use std::cell::RefCell;
use std::rc::Rc;

use crate::client_scripts::resource_manager::ResourceManager;
use crate::console::console_handler::Console;
use crate::controller::enums::controller_actions::ControllerActions;
use crate::debug::debug_info::DebugInfo;
use crate::logger::CONSOLE_LOGGER;
use crate::network::client::{NetworkContainer, NetworkLockType};
use crate::network::events::handle_network_events;
use crate::world::worlds_manager::WorldsManager;
use godot::engine::input::MouseMode;
use godot::engine::Engine;
use godot::prelude::*;
use network::client::IClientNetwork;
use network::messages::{ClientMessages, NetworkMessageType};

pub type FloatType = f32;
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(feature = "trace")]
#[global_allocator]
static GLOBAL: tracy_client::ProfiledAllocator<std::alloc::System> =
    tracy_client::ProfiledAllocator::new(std::alloc::System, 100);

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
    pub fn get_network_lock(&self) -> Option<NetworkLockType> {
        match self.network.as_ref() {
            Some(n) => Some(n.get_network_lock()),
            None => None,
        }
    }

    pub fn network_send_message(&self, message: &ClientMessages, message_type: NetworkMessageType) {
        let lock = self.get_network_lock().expect("network is not set");
        let network = lock.read();
        network.send_message(message, message_type);
    }

    pub fn get_resource_manager(&self) -> &ResourceManager {
        &self.resource_manager
    }

    pub fn get_resource_manager_mut(&mut self) -> &mut ResourceManager {
        &mut self.resource_manager
    }

    pub fn get_worlds_manager(&self) -> &WorldsManager {
        &self.worlds_manager
    }

    pub fn get_worlds_manager_mut(&mut self) -> &mut WorldsManager {
        &mut self.worlds_manager
    }

    pub fn close() {
        Engine::singleton()
            .get_main_loop()
            .expect("main loop is not found")
            .cast::<SceneTree>()
            .quit();
    }
}

#[godot_api]
impl INode for Main {
    fn init(base: Base<Node>) -> Self {
        let worlds_manager = WorldsManager::create(base.to_gd().clone());
        Main {
            base,
            network: None,
            resource_manager: Rc::new(RefCell::new(ResourceManager::new())),
            worlds_manager: worlds_manager,
            console: load::<PackedScene>("res://scenes/console.tscn").instantiate_as::<Console>(),
            debug_info: load::<PackedScene>("res://scenes/debug_info.tscn").instantiate_as::<DebugInfo>(),
        }
    }

    fn ready(&mut self) {
        if let Err(e) = log::set_logger(&CONSOLE_LOGGER) {
            log::error!(target: "main", "log::set_logger error: {}", e)
        }
        log::set_max_level(log::LevelFilter::Debug);

        log::info!(target: "main", "Start loading local resources");
        match self.get_resource_manager_mut().load_local_resources() {
            Ok(_) => (),
            Err(e) => {
                log::error!(target: "main", "Resources error: {}", e);
                Main::close();
                return;
            }
        }
        log::info!(target: "main", "Local resources loaded successfully ({})", self.get_resource_manager().get_resources_count());

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
        #[cfg(feature = "trace")]
        let _span = tracing::span!(tracing::Level::INFO, "main_scene").entered();

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
    }

    fn exit_tree(&mut self) {
        if let Some(n) = self.network.as_ref() {
            n.disconnect();
        }
        log::info!(target: "main", "Exiting the game");
    }
}
