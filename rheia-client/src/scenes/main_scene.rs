use crate::client_scripts::resource_manager::ResourceManager;
use crate::console::console_handler::Console;
use crate::controller::entity_movement::EntityMovement;
use crate::controller::enums::controller_actions::ControllerActions;
use crate::controller::player_action::{PlayerAction, PlayerActionType};
use crate::debug::debug_info::DebugInfo;
use crate::logger::CONSOLE_LOGGER;
use crate::network::client::NetworkContainer;
use crate::network::events::handle_network_events;
use crate::ui::block_selection::BlockSelection;
use crate::utils::world_generator::generate_chunks;
use crate::world::physics::PhysicsType;
use crate::world::worlds_manager::WorldsManager;
use common::blocks::block_info::BlockInfo;
use common::chunks::rotation::Rotation;
use common::world_generator::default::WorldGeneratorSettings;
use godot::classes::file_access::ModeFlags;
use godot::classes::input::MouseMode;
use godot::classes::{Engine, FileAccess};
use godot::prelude::*;
use network::messages::{ClientMessages, NetworkMessageType};
use std::cell::RefCell;
use std::rc::Rc;

use crate::scenes::text_screen::TextScreen;

pub type FloatType = f32;

#[cfg(feature = "trace")]
#[global_allocator]
static GLOBAL: tracy_client::ProfiledAllocator<std::alloc::System> =
    tracy_client::ProfiledAllocator::new(std::alloc::System, 100);

pub type ResourceManagerType = Rc<RefCell<ResourceManager>>;

#[derive(GodotClass)]
#[class(init, tool, base=Node)]
pub struct MainScene {
    base: Base<Node>,
    ip_port: Option<String>,
    login: Option<String>,

    network: Option<NetworkContainer>,

    resource_manager: ResourceManagerType,

    #[export]
    worlds_manager: Option<Gd<WorldsManager>>,

    #[init(val = OnReady::manual())]
    console: OnReady<Gd<Console>>,
    #[export]
    console_scene: Option<Gd<PackedScene>>,

    #[init(val = OnReady::manual())]
    text_screen: OnReady<Gd<TextScreen>>,
    #[export]
    text_screen_scene: Option<Gd<PackedScene>>,

    #[init(val = OnReady::manual())]
    debug_info: OnReady<Gd<DebugInfo>>,
    #[export]
    debug_info_scene: Option<Gd<PackedScene>>,

    #[export]
    block_icon_scene: Option<Gd<PackedScene>>,

    #[init(val = OnReady::manual())]
    block_selection: OnReady<Gd<BlockSelection>>,
    #[export]
    block_selection_scene: Option<Gd<PackedScene>>,

    #[var(usage_flags = [GROUP, EDITOR, READ_ONLY])]
    debug_world: u32,

    #[var(get, set = regenerate_debug_world)]
    #[export]
    regenerate_map_button: bool,

    #[init(val = 12)]
    #[export]
    debug_render_distance: u16,

    #[export(file = "*.json")]
    debug_world_settings: GString,
}

impl MainScene {
    pub fn init_data(&mut self, ip_port: String, login: String) {
        self.ip_port = Some(ip_port);
        self.login = Some(login);
    }

    pub fn get_network(&self) -> Option<&NetworkContainer> {
        self.network.as_ref()
    }

    pub fn get_login(&self) -> &String {
        self.login.as_ref().expect("init_data is not called")
    }

    pub fn get_text_screen_mut(&mut self) -> GdMut<'_, TextScreen> {
        self.text_screen.bind_mut()
    }

    pub fn get_resource_manager(&self) -> std::cell::Ref<'_, ResourceManager> {
        self.resource_manager.borrow()
    }

    pub fn get_resource_manager_mut(&self) -> std::cell::RefMut<'_, ResourceManager> {
        self.resource_manager.borrow_mut()
    }

    pub fn get_wm(&self) -> &Gd<WorldsManager> {
        self.worlds_manager.as_ref().unwrap()
    }

    pub fn get_worlds_manager_mut(&mut self) -> GdMut<'_, WorldsManager> {
        self.worlds_manager.as_mut().unwrap().bind_mut()
    }

    fn connect(&mut self) {
        let ip = self.ip_port.as_ref().expect("init_data is not called");

        self.text_screen
            .bind_mut()
            .update_text(format!("Connecting to {}...", ip));

        let network = match NetworkContainer::new(ip.clone()) {
            Ok(c) => c,
            Err(e) => {
                self.send_disconnect_event(format!("Connection error: {}", e));
                return;
            }
        };
        network.spawn_network_thread();
        self.network = Some(network);
    }

    pub fn send_disconnect_event(&mut self, message: String) {
        Input::singleton().set_mouse_mode(MouseMode::VISIBLE);
        self.base_mut().emit_signal("disconnect", &[message.to_variant()]);
    }

    /// Signaling that everything is loaded from the server
    pub fn on_server_connected(&mut self) {
        self.debug_info.bind_mut().toggle(true);

        let resource_manager = self.get_resource_manager();

        let worlds_manager = self.get_wm().clone();
        let worlds_manager = worlds_manager.bind();
        let block_storage = worlds_manager.get_block_storage();

        let mut block_selection = self.block_selection.clone();
        block_selection
            .bind_mut()
            .init_blocks(&*block_storage, &*resource_manager);
    }

    /// Player can teleport in new world, between worlds or in exsting world
    /// so worlds can be created and destroyed
    pub fn teleport_player(&mut self, world_slug: String, position: Vector3, rotation: Rotation) {
        let base = self.base().clone();
        let mut worlds_manager = self.get_worlds_manager_mut();

        let created_controller = if let Some(world) = worlds_manager.get_world() {
            if world.bind().get_slug() != &world_slug {
                // Player moving to another world; old one must be destroyed
                worlds_manager.destroy_world();
                let world = worlds_manager.create_world(world_slug);
                Some(worlds_manager.create_player(&world))
            } else {
                None
            }
        } else {
            let world = worlds_manager.create_world(world_slug);
            Some(worlds_manager.create_player(&world))
        };

        if let Some(mut player_controller) = created_controller {
            player_controller.bind_mut().base_mut().connect(
                "on_player_move",
                &Callable::from_object_method(&base, "handler_player_move"),
            );
            player_controller.bind_mut().base_mut().connect(
                "on_player_action",
                &Callable::from_object_method(&base, "handler_player_action"),
            );
        }

        worlds_manager.teleport_player_controller(position, rotation)
    }
}

#[godot_api]
impl MainScene {
    #[signal]
    fn disconnect();

    #[func]
    fn handler_player_move(&mut self, movement: Gd<EntityMovement>, _new_chunk: bool) {
        let network = self.get_network().unwrap();
        network.send_message(NetworkMessageType::Unreliable, &movement.bind().into_network());
    }

    #[func]
    fn regenerate_debug_world(&mut self, _value: bool) {
        log::info!("Regenerate debug world");

        let wm = self.worlds_manager.as_mut().expect("worlds_manager is not init");

        let rm = self.resource_manager.borrow();
        let resources_storage = rm.get_resources_storage();
        wm.bind_mut().build_textures(&*resources_storage).unwrap();

        if wm.bind().get_world().is_some() {
            wm.bind_mut().destroy_world();
        }

        let mut world = wm.bind_mut().create_world(String::from("TestWorld"));

        let settings_file = FileAccess::open(&self.debug_world_settings.to_string(), ModeFlags::READ).unwrap();
        let settings_value: serde_json::Value = serde_json::from_str(&settings_file.get_as_text().to_string()).unwrap();

        let settings: WorldGeneratorSettings = match serde_json::from_value(settings_value) {
            Ok(s) => s,
            Err(e) => panic!("Settings json error: {}", e),
        };

        generate_chunks(&mut world, 0, 0, self.debug_render_distance, settings);
    }

    #[func]
    fn handler_player_action(&mut self, action: Gd<PlayerAction>) {
        let a = action.bind();
        let network = self.get_network().unwrap();
        if let Some((cast_result, physics_type)) = a.get_hit() {
            match physics_type {
                PhysicsType::ChunkMeshCollider(_chunk_position) => {
                    let selected_block = cast_result.get_selected_block();
                    let msg = match a.get_action_type() {
                        PlayerActionType::Main => ClientMessages::EditBlockRequest {
                            world_slug: a.get_world_slug().clone(),
                            position: cast_result.get_place_block(),
                            new_block_info: Some(BlockInfo::create(100, None)),
                        },
                        PlayerActionType::Second => ClientMessages::EditBlockRequest {
                            world_slug: a.get_world_slug().clone(),
                            position: selected_block,
                            new_block_info: None,
                        },
                    };
                    network.send_message(NetworkMessageType::Unreliable, &msg);
                }
                PhysicsType::EntityCollider(_entity_id) => {}
            }
        }
    }
}

#[godot_api]
impl INode for MainScene {
    fn ready(&mut self) {
        log::set_max_level(log::LevelFilter::Debug);

        log::info!(target: "main", "Start loading local resources");
        if let Err(e) = self.resource_manager.clone().borrow_mut().load_local_resources() {
            self.send_disconnect_event(format!("Internal resources error: {}", e));
            return;
        }
        log::info!(target: "main", "Local resources loaded successfully (count: {})", self.get_resource_manager().get_resources_storage().get_resources_count());

        if Engine::singleton().is_editor_hint() {
            if let Err(e) = log::set_logger(&CONSOLE_LOGGER) {
                log::error!(target: "main", "log::set_logger error: {}", e)
            }

            self.regenerate_debug_world(false);
        } else {
            // Console
            let console = self.console_scene.as_mut().unwrap().instantiate_as::<Console>();
            self.console.init(console);

            let console = self.console.clone();
            self.base_mut().add_child(&console);

            // Debug
            let debug_info = self.debug_info_scene.as_mut().unwrap().instantiate_as::<DebugInfo>();
            self.debug_info.init(debug_info);

            let debug_info = self.debug_info.clone();
            self.base_mut().add_child(&debug_info);

            self.debug_info.bind_mut().toggle(false);

            // Selection meny
            let block_selection = self
                .block_selection_scene
                .as_mut()
                .unwrap()
                .instantiate_as::<BlockSelection>();
            self.block_selection.init(block_selection);

            let block_selection = self.block_selection.clone();
            self.base_mut().add_child(&block_selection);

            // Text splash screen
            let text_screen = self.text_screen_scene.as_mut().unwrap().instantiate_as::<TextScreen>();
            self.text_screen.init(text_screen);

            let text_screen = self.text_screen.clone();
            self.base_mut().add_child(&text_screen);
            self.text_screen.bind_mut().toggle(true);

            Input::singleton().set_mouse_mode(MouseMode::CAPTURED);

            self.connect();
        }

        if let Some(worlds_manager) = self.worlds_manager.as_mut() {
            worlds_manager
                .bind_mut()
                .set_resource_manager(self.resource_manager.clone());
        }
    }

    fn process(&mut self, _delta: f64) {
        #[cfg(feature = "trace")]
        let _span = tracing::span!(tracing::Level::INFO, "main_scene").entered();

        if self.network.is_some() {
            let network_info = match handle_network_events(self) {
                Ok(i) => i,
                Err(e) => {
                    self.send_disconnect_event(format!("Network error: {}", e));
                    return;
                }
            };

            let wm = self.worlds_manager.as_ref().unwrap();
            self.debug_info.bind_mut().update_debug(wm, network_info);
        }

        if !Engine::singleton().is_editor_hint() {
            let input = Input::singleton();
            if input.is_action_just_pressed(&ControllerActions::ToggleConsole.to_string()) {
                self.console.bind_mut().toggle(!Console::is_active());

                if Console::is_active() {
                    self.debug_info.bind_mut().toggle(false);
                }
            }
            if input.is_action_just_pressed(&ControllerActions::ToggleDebug.to_string()) {
                self.debug_info.bind_mut().toggle(!DebugInfo::is_active());

                if DebugInfo::is_active() {
                    self.console.bind_mut().toggle(false);
                }
            }
        }
    }

    fn exit_tree(&mut self) {
        log::info!(target: "main", "Exiting the game");
        if let Some(n) = self.network.as_ref() {
            n.disconnect();
        }
    }
}
