use std::cell::RefCell;
use std::rc::Rc;

use crate::client_scripts::resource_manager::ResourceManager;
use crate::console::console_handler::Console;
use crate::controller::enums::controller_actions::ControllerActions;
use crate::debug::debug_info::DebugInfo;
use crate::network::client::{NetworkContainer, NetworkLockType};
use crate::network::events::handle_network_events;
use crate::world::worlds_manager::WorldsManager;
use godot::engine::input::MouseMode;
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
#[class(base=Node)]
pub struct MainScene {
    base: Base<Node>,
    ip_port: Option<String>,

    network: Option<NetworkContainer>,

    resource_manager: ResourceManager,
    worlds_manager: Rc<RefCell<WorldsManager>>,
    console: Gd<Console>,
    debug_info: Gd<DebugInfo>,
    text_screen: Gd<TextScreen>,
}

impl MainScene {
    pub fn set_ip(&mut self, ip_port: String) {
        self.ip_port = Some(ip_port);
    }

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

    pub fn get_text_screen_mut(&mut self) -> GdMut<'_, TextScreen> {
        self.text_screen.bind_mut()
    }

    pub fn get_resource_manager(&self) -> &ResourceManager {
        &self.resource_manager
    }

    pub fn get_resource_manager_mut(&mut self) -> &mut ResourceManager {
        &mut self.resource_manager
    }

    pub fn _get_worlds_manager(&self) -> std::cell::Ref<'_, WorldsManager> {
        self.worlds_manager.borrow()
    }

    pub fn get_worlds_manager_mut(&self) -> std::cell::RefMut<'_, WorldsManager> {
        self.worlds_manager.borrow_mut()
    }

    fn connect(&mut self) {
        let ip = self.ip_port.as_ref().expect("set_ip is not called");

        self.text_screen.bind_mut().set_text(format!("Connecting to {}...", ip));

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
        self.base_mut()
            .emit_signal("disconnect".into(), &[message.to_variant()]);
    }
}

#[godot_api]
impl MainScene {
    #[signal]
    fn disconnect();
}

#[godot_api]
impl INode for MainScene {
    fn init(base: Base<Node>) -> Self {
        let worlds_manager = WorldsManager::create(base.to_gd().clone());
        Self {
            base,
            ip_port: None,
            network: None,
            resource_manager: ResourceManager::new(),
            worlds_manager: Rc::new(RefCell::new(worlds_manager)),
            console: load::<PackedScene>("res://scenes/console.tscn").instantiate_as::<Console>(),
            debug_info: load::<PackedScene>("res://scenes/debug_info.tscn").instantiate_as::<DebugInfo>(),
            text_screen: load::<PackedScene>("res://scenes/text_screen.tscn").instantiate_as::<TextScreen>(),
        }
    }

    fn ready(&mut self) {
        log::info!(target: "main", "Start loading local resources");
        if let Err(e) = self.get_resource_manager_mut().load_local_resources() {
            self.send_disconnect_event(format!("Internal resources error: {}", e));
            return;
        }
        log::info!(target: "main", "Local resources loaded successfully ({})", self.get_resource_manager().get_resources_count());

        let console = self.console.clone().upcast();
        self.base_mut().add_child(console);

        let debug_info = self.debug_info.clone().upcast();
        self.base_mut().add_child(debug_info);

        self.debug_info.bind_mut().toggle(false);

        let text_screen = self.text_screen.clone().upcast();
        self.base_mut().add_child(text_screen);
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

            let wm = self.worlds_manager.clone();
            let worlds_manager = wm.borrow();
            self.debug_info.bind_mut().update_debug(&worlds_manager, network_info);
        }

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