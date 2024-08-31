use crate::client_scripts::resource_manager::ResourceManager;
use crate::console::console_handler::Console;
use crate::controller::enums::controller_actions::ControllerActions;
use crate::debug::debug_info::DebugInfo;
use crate::logger::CONSOLE_LOGGER;
use crate::network::client::{NetworkContainer, NetworkLockType};
use crate::network::events::handle_network_events;
use crate::world::worlds_manager::WorldsManager;
use common::network::client::ClientNetwork;
use common::network::messages::{ClientMessages, NetworkMessageType};
use godot::engine::input::MouseMode;
use godot::engine::Engine;
use godot::prelude::*;

pub type FloatType = f32;
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    base: Base<Node>,

    network: Option<NetworkContainer>,

    resource_manager: ResourceManager,
    worlds_manager: WorldsManager,
    console: Gd<Console>,
    debug_info: Gd<DebugInfo>,
}

impl Main {
    pub fn get_network_lock(&self) -> NetworkLockType {
        self.network.as_ref().unwrap().get_network_lock()
    }

    pub fn network_send_message(&self, message: &ClientMessages, message_type: NetworkMessageType) {
        let lock = self.get_network_lock();
        let network = lock.read();
        network.send_message(message, message_type);
    }

    pub fn get_resource_manager_mut(&mut self) -> &mut ResourceManager {
        &mut self.resource_manager
    }

    pub fn get_worlds_manager_mut(&mut self) -> &mut WorldsManager {
        &mut self.worlds_manager
    }

    pub fn close() {
        Engine::singleton().get_main_loop().unwrap().cast::<SceneTree>().quit();
    }
}

#[godot_api]
impl INode for Main {
    fn init(base: Base<Node>) -> Self {
        let worlds_manager = WorldsManager::create(base.to_gd().clone());
        Main {
            base,
            network: None,
            resource_manager: ResourceManager::new(),
            worlds_manager: worlds_manager,
            console: load::<PackedScene>("res://scenes/console.tscn").instantiate_as::<Console>(),
            debug_info: load::<PackedScene>("res://scenes/debug_info.tscn").instantiate_as::<DebugInfo>(),
        }
    }

    fn ready(&mut self) {
        log::set_logger(&CONSOLE_LOGGER).unwrap();
        log::set_max_level(log::LevelFilter::Debug);

        let console = self.console.clone().upcast();
        self.base_mut().add_child(console);

        let debug_info = self.debug_info.clone().upcast();
        self.base_mut().add_child(debug_info);

        self.debug_info.bind_mut().toggle(true);

        log::info!(target: "main", "Loading Rheia version: {}", VERSION);

        let ip_port = "127.0.0.1:19132".to_string();
        let network = match NetworkContainer::new(ip_port) {
            Ok(c) => c,
            Err(e) => {
                log::error!(target: "main", "Network connection error: {}", e);
                Main::close();
                return;
            }
        };

        self.network = Some(network);

        Input::singleton().set_mouse_mode(MouseMode::CAPTURED);
    }

    fn process(&mut self, _delta: f64) {
        let now = std::time::Instant::now();

        let network_info = handle_network_events(self);

        self.debug_info
            .bind_mut()
            .update_debug(&self.worlds_manager, network_info);

        let input = Input::singleton();
        if input.is_action_just_pressed(ControllerActions::ToggleConsole.to_string().into()) {
            self.console.bind_mut().toggle(!Console::is_active());

            if Console::is_active() {
                self.debug_info.bind_mut().toggle(false);
            }
        }
        if input.is_action_just_pressed(ControllerActions::ToggleDebug.to_string().into()) {
            self.debug_info.bind_mut().toggle(!DebugInfo::is_active());

            if DebugInfo::is_active() {
                self.console.bind_mut().toggle(false);
            }
        }

        let elapsed = now.elapsed();
        if elapsed > std::time::Duration::from_millis(20) {
            log::debug!(target: "main", "Main process: {:.2?}", elapsed);
        }
    }

    fn exit_tree(&mut self) {
        if let Some(n) = self.network.as_ref() {
            n.disconnect();
        }
        log::info!(target: "main", "Exiting the game");
    }
}
