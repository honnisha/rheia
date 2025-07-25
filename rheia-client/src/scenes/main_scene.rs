use crate::client_scripts::resource_manager::ResourceManager;
use crate::console::console_handler::{Console, GDCommandMatch};
use crate::controller::entity_movement::EntityMovement;
use crate::controller::enums::controller_actions::ControllerActions;
use crate::controller::player_action::PlayerAction;
use crate::controller::selected_item::{SelectedItem, SelectedItemGd};
use crate::debug::debug_info::DebugInfo;
use crate::logger::CONSOLE_LOGGER;
use crate::network::client::NetworkContainer;
use crate::network::events::handle_network_events;
use crate::scenes::text_screen::TextScreen;
use crate::utils::settings::GameSettings;
use crate::utils::world_generator::generate_chunks;
use crate::world::physics::PhysicsType;
use crate::world::worlds_manager::WorldsManager;
use crate::{LOG_LEVEL, MAX_THREADS};
use common::blocks::block_info::generate_block_id_map;
use common::chunks::chunk_data::BlockIndexType;
use common::world_generator::default::WorldGeneratorSettings;
use godot::classes::file_access::ModeFlags;
use godot::classes::input::MouseMode;
use godot::classes::{Engine, FileAccess, Input, WorldEnvironment};
use godot::prelude::*;
use network::messages::{ClientMessages, NetworkMessageType};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

pub type FloatType = f32;

const MAIN_SCENE_PATH: &str = "res://scenes/main_scene.tscn";
pub const DEFAULT_THEME_PATH: &str = "res://assets/gui/default_theme.tres";

pub type ResourceManagerType = Rc<RefCell<ResourceManager>>;

#[derive(GodotClass)]
#[class(init, tool, base=Node)]
pub struct MainScene {
    pub(crate) base: Base<Node>,
    ip_port: Option<String>,
    login: Option<String>,

    network: Option<NetworkContainer>,

    resource_manager: ResourceManagerType,

    #[export]
    worlds_manager: Option<Gd<WorldsManager>>,

    #[init(val = OnReady::manual())]
    console: OnReady<Gd<Console>>,

    #[init(val = OnReady::manual())]
    text_screen: OnReady<Gd<TextScreen>>,
    #[export]
    text_screen_scene: Option<Gd<PackedScene>>,

    #[init(val = OnReady::manual())]
    debug_info: OnReady<Gd<DebugInfo>>,
    #[export]
    debug_info_scene: Option<Gd<PackedScene>>,

    #[var(usage_flags = [GROUP, EDITOR, READ_ONLY])]
    debug_world: u32,

    #[var(get, set = regenerate_debug_world)]
    #[export]
    regenerate_map_button: bool,

    #[init(val = 12)]
    #[export]
    debug_render_distance: i32,

    #[export(file = "*")]
    debug_world_settings: GString,

    game_settings: Option<Rc<RefCell<GameSettings>>>,

    #[export]
    worlde_environment: Option<Gd<WorldEnvironment>>,
}

impl MainScene {
    pub fn create(ip_port: String, login: String, game_settings: Rc<RefCell<GameSettings>>) -> Gd<Self> {
        rayon::ThreadPoolBuilder::new()
            .num_threads(MAX_THREADS)
            .build_global()
            .unwrap();

        let mut scene = load::<PackedScene>(MAIN_SCENE_PATH).instantiate_as::<Self>();
        scene.bind_mut().ip_port = Some(ip_port);
        scene.bind_mut().login = Some(login);
        scene.bind_mut().game_settings = Some(game_settings);
        scene
    }

    pub fn get_network(&self) -> Option<&NetworkContainer> {
        self.network.as_ref()
    }

    pub fn get_login(&self) -> &String {
        self.login.as_ref().unwrap()
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

    fn connect_to_server(&mut self) {
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
        self.signals().network_disconnect().emit(&message.to_godot());
    }

    /// Signaling that everything is loaded from the server
    pub fn on_server_connected(&mut self) {
        self.debug_info.bind_mut().toggle(true);

        let rm = self.resource_manager.clone();
        self.get_worlds_manager_mut().on_network_connected(&*rm.borrow());
    }

    /// Player can teleport in new world, between worlds or in exsting world
    /// so worlds can be created and destroyed
    pub fn spawn_world(&mut self, world_slug: String) {
        let mut wm = self.worlds_manager.as_mut().unwrap().clone();
        let mut worlds_manager = wm.bind_mut();

        let created_controller = if let Some(world) = worlds_manager.get_world() {
            if world.bind().get_slug() != &world_slug {
                // Player moving to another world; old one must be destroyed
                worlds_manager.destroy_world();
                let world = worlds_manager.create_world(world_slug);
                Some(worlds_manager.create_player(&world))
            } else {
                // The same world
                None
            }
        } else {
            // New world
            let world = worlds_manager.create_world(world_slug);
            Some(worlds_manager.create_player(&world))
        };

        if let Some(mut player_controller) = created_controller {
            player_controller
                .signals()
                .player_move()
                .connect_other(&self.to_gd(), MainScene::handler_player_move);

            player_controller
                .signals()
                .player_action()
                .connect_other(&self.to_gd(), MainScene::handler_player_action);

            player_controller.bind_mut().set_blocks(&*worlds_manager);
        }
    }
}

#[godot_api]
impl MainScene {
    #[signal]
    pub fn network_disconnect(message: GString);

    #[func]
    fn handler_player_move(&mut self, movement: Gd<EntityMovement>, _new_chunk: bool) {
        let network = self.get_network().unwrap();
        network.send_message(NetworkMessageType::Unreliable, &movement.bind().into_network());
    }

    #[func]
    fn on_client_command_sended(&mut self, command: Gd<GDCommandMatch>) {
        let command = command.bind().command_match.clone();

        if *command.get_name() == "exit" {
            log::info!(target: "main", "&cClosing the game...");
            if let Some(n) = self.network.as_ref() {
                n.disconnect();
            }
            Engine::singleton()
                .get_main_loop()
                .expect("main loop is not found")
                .cast::<SceneTree>()
                .quit();
            return;
        }

        if *command.get_name() == "disconnect" {
            log::info!(target: "main", "&cDisconnecting from the server...");
            if let Some(n) = self.network.as_ref() {
                n.disconnect();
            }
            return;
        }

        if *command.get_name() == "setting" {
            let game_settings = self.game_settings.as_ref().unwrap();
            let mut settings = game_settings.borrow_mut();

            let setting_type = match command.get_arg::<String, _>("name") {
                Ok(c) => c,
                Err(e) => {
                    log::error!(target: "main", "&cSetting type error: {}", e);
                    return;
                }
            };
            match setting_type.as_str() {
                "ssao" => {
                    let value = match command.get_arg::<bool, _>("value") {
                        Ok(c) => c,
                        Err(e) => {
                            log::error!(target: "main", "&cSetting value error: {}", e);
                            return;
                        }
                    };
                    settings.ssao = value;
                    settings.save().unwrap();
                    let worlde_environment = self.worlde_environment.as_mut().unwrap();
                    let mut environment = worlde_environment.get_environment().unwrap();
                    environment.set_ssao_enabled(settings.ssao);
                    log::info!(target: "main", "&aSetting SSAO changed to &2{}", settings.ssao);
                    return;
                }
                "fps" => {
                    let value = match command.get_arg::<u16, _>("value") {
                        Ok(c) => c,
                        Err(e) => {
                            log::error!(target: "main", "&cSetting value error: {}", e);
                            return;
                        }
                    };
                    settings.fps = value;
                    Engine::singleton().set_max_fps(settings.fps as i32);
                    settings.save().unwrap();
                    log::info!(target: "main", "&aSetting FPS changed to &2{}", settings.fps);
                    return;
                }
                _ => {
                    log::error!(target: "main", "&cSetting type \"{}\" not found", setting_type.as_str());
                    return;
                }
            }
        }
        log::info!(target: "main", "&cCommand &4\"{}\" &cis not handeled by client", command.get_name());
    }

    #[func]
    fn on_network_command_sended(&mut self, command: GString) {
        let network = self.get_network().unwrap();
        let message = ClientMessages::ConsoleInput {
            command: command.to_string(),
        };
        network.send_message(NetworkMessageType::ReliableOrdered, &message);
    }

    #[func]
    fn regenerate_debug_world(&mut self, _value: bool) {
        log::info!(target: "main", "Regenerate debug world");

        let Some(settings_file) = FileAccess::open(&self.debug_world_settings.to_string(), ModeFlags::READ) else {
            log::error!(
                "World settings file {} not found",
                self.debug_world_settings.to_string()
            );
            return;
        };
        let settings: WorldGeneratorSettings = match serde_yaml::from_str(&settings_file.get_as_text().to_string()) {
            Ok(s) => s,
            Err(e) => {
                log::error!("World settings yaml error: {}", e);
                return;
            }
        };

        let wm = self.worlds_manager.as_mut().expect("worlds_manager is not init");

        {
            let wm = wm.bind_mut();
            let mut block_storage = wm.get_block_storage_mut();
            let mut block_id_map: BTreeMap<BlockIndexType, String> = Default::default();
            let _ = generate_block_id_map(&mut block_id_map, block_storage.iter_values());
            block_storage.set_block_id_map(block_id_map);
        }

        let rm = self.resource_manager.borrow();
        let resources_storage = rm.get_resources_storage();
        wm.bind_mut().build_textures(&*resources_storage).unwrap();

        if wm.bind().get_world().is_some() {
            wm.bind_mut().destroy_world();
        }

        let mut world = wm.bind_mut().create_world(String::from("TestWorld"));
        generate_chunks(&mut world, 0, 0, self.debug_render_distance, settings);
    }

    #[func]
    fn handler_player_action(&mut self, action: Gd<PlayerAction>, item: Gd<SelectedItemGd>) {
        let a = action.bind();
        let network = self.get_network().unwrap();
        if let Some(look_at) = a.get_hit() {
            match look_at.bind().get_physics_type() {
                PhysicsType::ChunkMeshCollider(_chunk_position) => {
                    let selected_block = look_at.bind().get_cast_result().get_selected_block();
                    if a.is_main_type() {
                        if let Some(i) = item.bind().get_selected_item() {
                            match i {
                                SelectedItem::BlockPlacing(block_info) => {
                                    let msg = ClientMessages::EditBlockRequest {
                                        world_slug: a.get_world_slug().clone(),
                                        position: look_at.bind().get_cast_result().get_place_block(),
                                        new_block_info: Some(block_info.clone()),
                                    };
                                    network.send_message(NetworkMessageType::Unreliable, &msg);
                                }
                            }
                        }
                    } else {
                        let msg = ClientMessages::EditBlockRequest {
                            world_slug: a.get_world_slug().clone(),
                            position: selected_block,
                            new_block_info: None,
                        };
                        network.send_message(NetworkMessageType::Unreliable, &msg);
                    }
                }
                PhysicsType::EntityCollider(_entity_id) => {}
            }
        }
    }
}

#[godot_api]
impl INode for MainScene {
    fn ready(&mut self) {
        let gd = self.to_gd().clone();
        log::set_max_level(LOG_LEVEL);
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
            // Debug
            let debug_info = self.debug_info_scene.as_mut().unwrap().instantiate_as::<DebugInfo>();
            self.debug_info.init(debug_info);

            let debug_info = self.debug_info.clone();
            self.base_mut().add_child(&debug_info);

            // Console
            let console = Console::create();
            console
                .signals()
                .network_command_sended()
                .connect_other(&gd, Self::on_network_command_sended);
            console
                .signals()
                .client_command_sended()
                .connect_other(&gd, Self::on_client_command_sended);
            self.console.init(console);

            let console = self.console.clone();
            self.base_mut().add_child(&console);

            self.debug_info.bind_mut().toggle(false);

            // Text splash screen
            let text_screen = self.text_screen_scene.as_mut().unwrap().instantiate_as::<TextScreen>();
            self.text_screen.init(text_screen);

            let text_screen = self.text_screen.clone();
            self.base_mut().add_child(&text_screen);
            self.text_screen.bind_mut().toggle(true);

            Input::singleton().set_mouse_mode(MouseMode::CAPTURED);

            self.connect_to_server();
        }

        if let Some(game_settings) = self.game_settings.as_ref() {
            let settings = game_settings.borrow();
            if let Some(worlde_environment) = self.worlde_environment.as_mut() {
                let mut environment = worlde_environment.get_environment().unwrap();
                environment.set_ssao_enabled(settings.ssao);
            }
            Engine::singleton().set_max_fps(settings.fps as i32);
        }

        let mut wm = self.worlds_manager.clone();
        if let Some(worlds_manager) = wm.as_mut() {
            worlds_manager
                .bind_mut()
                .set_resource_manager(self.resource_manager.clone());
        }
    }

    fn process(&mut self, _delta: f64) {
        #[cfg(feature = "trace")]
        let _span = tracy_client::span!("main_scene");

        let now = std::time::Instant::now();

        if self.network.is_some() {
            let network_info = match handle_network_events(self) {
                Ok(i) => i,
                Err(e) => {
                    log::error!(target: "main", "Network error: {}", e);
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
            }
            if input.is_action_just_pressed(&ControllerActions::ToggleDebug.to_string()) {
                self.debug_info.bind_mut().toggle(!DebugInfo::is_active());
            }
        }

        let elapsed = now.elapsed();
        #[cfg(debug_assertions)]
        if elapsed >= crate::WARNING_TIME {
            log::warn!(target: "main", "&7process lag: {:.2?}", elapsed);
        }
    }

    fn exit_tree(&mut self) {
        {
            let mut worlds_manager = self.get_worlds_manager_mut();
            if worlds_manager.get_world().is_some() {
                worlds_manager.destroy_world()
            }
        }
        if let Some(n) = self.network.as_ref() {
            n.disconnect();
        }
        log::info!(target: "main", "Main scene exited;");
    }
}
