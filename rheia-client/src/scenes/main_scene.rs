use crate::client_scripts::resource_manager::ResourceManager;
use crate::console::console_handler::Console;
use crate::controller::entity_movement::EntityMovement;
use crate::controller::enums::controller_actions::ControllerActions;
use crate::controller::player_action::{PlayerAction, PlayerActionType};
use crate::debug::debug_info::DebugInfo;
use crate::network::client::{NetworkContainer, NetworkLockType};
use crate::network::events::handle_network_events;
use crate::world::physics::PhysicsType;
use crate::world::worlds_manager::WorldsManager;
use common::blocks::block_info::BlockInfo;
use common::chunks::rotation::Rotation;
use godot::classes::input::MouseMode;
use godot::prelude::*;
use network::client::IClientNetwork;
use network::messages::{ClientMessages, NetworkMessageType};

use crate::scenes::text_screen::TextScreen;

pub type FloatType = f32;

#[cfg(feature = "trace")]
#[global_allocator]
static GLOBAL: tracy_client::ProfiledAllocator<std::alloc::System> =
    tracy_client::ProfiledAllocator::new(std::alloc::System, 100);

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct MainScene {
    base: Base<Node>,
    ip_port: Option<String>,
    login: Option<String>,

    network: Option<NetworkContainer>,

    resource_manager: ResourceManager,

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
}

impl MainScene {
    pub fn init_data(&mut self, ip_port: String, login: String) {
        self.ip_port = Some(ip_port);
        self.login = Some(login);
    }

    pub fn get_network_lock(&self) -> Option<NetworkLockType> {
        match self.network.as_ref() {
            Some(n) => Some(n.get_network_lock()),
            None => None,
        }
    }

    pub fn get_login(&self) -> &String {
        self.login.as_ref().expect("init_data is not called")
    }

    pub fn get_text_screen_mut(&mut self) -> GdMut<'_, TextScreen> {
        self.text_screen.bind_mut()
    }

    pub fn get_resource_manager(&self) -> &ResourceManager {
        &self.resource_manager
    }

    pub fn get_resource_manager_mut(&mut self) -> &mut ResourceManager {
        &mut self.resource_manager
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
        self.network = Some(network);
    }

    pub fn send_disconnect_event(&mut self, message: String) {
        Input::singleton().set_mouse_mode(MouseMode::VISIBLE);
        self.base_mut().emit_signal("disconnect", &[message.to_variant()]);
    }

    pub fn on_server_connected(&mut self) {
        self.debug_info.bind_mut().toggle(true);
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
        let network_lock = self.get_network_lock().unwrap();
        network_lock
            .read()
            .send_message(NetworkMessageType::Unreliable, &movement.bind().into_network());
    }

    #[func]
    fn handler_player_action(&mut self, action: Gd<PlayerAction>) {
        let a = action.bind();
        let network_lock = self.get_network_lock().unwrap();
        if let Some((cast_result, physics_type)) = a.get_hit() {
            match physics_type {
                PhysicsType::ChunkMeshCollider(_chunk_position) => {
                    let selected_block = cast_result.get_selected_block();
                    let msg = match a.get_action_type() {
                        PlayerActionType::Main => ClientMessages::EditBlockRequest {
                            world_slug: a.get_world_slug().clone(),
                            position: cast_result.get_place_block(),
                            new_block_info: BlockInfo::create(1, None),
                        },
                        PlayerActionType::Second => ClientMessages::EditBlockRequest {
                            world_slug: a.get_world_slug().clone(),
                            position: selected_block,
                            new_block_info: BlockInfo::create(0, None),
                        },
                    };
                    network_lock.read().send_message(NetworkMessageType::Unreliable, &msg);
                }
                PhysicsType::EntityCollider(_entity_id) => {}
            }
        }
    }
}

#[godot_api]
impl INode for MainScene {
    fn ready(&mut self) {
        let console = self.console_scene.as_mut().unwrap().instantiate_as::<Console>();
        self.console.init(console);

        let debug_info = self.debug_info_scene.as_mut().unwrap().instantiate_as::<DebugInfo>();
        self.debug_info.init(debug_info);

        let text_screen = self.text_screen_scene.as_mut().unwrap().instantiate_as::<TextScreen>();
        self.text_screen.init(text_screen);

        log::info!(target: "main", "Start loading local resources");
        if let Err(e) = self.get_resource_manager_mut().load_local_resources() {
            self.send_disconnect_event(format!("Internal resources error: {}", e));
            return;
        }
        log::info!(target: "main", "Local resources loaded successfully ({})", self.get_resource_manager().get_resources_count());

        let console = self.console.clone();
        self.base_mut().add_child(&console);

        let debug_info = self.debug_info.clone();
        self.base_mut().add_child(&debug_info);

        self.debug_info.bind_mut().toggle(false);

        let text_screen = self.text_screen.clone();
        self.base_mut().add_child(&text_screen);
        self.text_screen.bind_mut().toggle(true);

        Input::singleton().set_mouse_mode(MouseMode::CAPTURED);

        self.connect();
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

    fn exit_tree(&mut self) {
        if let Some(n) = self.network.as_ref() {
            n.disconnect();
        }
        log::info!(target: "main", "Exiting the game");
    }
}
