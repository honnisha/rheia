use std::cell::RefCell;
use std::rc::Rc;

use crate::client_scripts::resource_manager::ResourceManager;
use crate::console::console_handler::Console;
use crate::controller::enums::controller_actions::ControllerActions;
use crate::debug::debug_info::DebugInfo;
use crate::network::client::{NetworkContainer, NetworkLockType};
use crate::network::events::handle_network_events;
use crate::world::worlds_manager::WorldsManager;
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

    network: Option<NetworkContainer>,

    resource_manager: ResourceManager,

    #[init(val = OnReady::manual())]
    worlds_manager: OnReady<Rc<RefCell<WorldsManager>>>,

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

        self.text_screen.bind_mut().update_text(format!("Connecting to {}...", ip));

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
}

#[godot_api]
impl MainScene {
    #[signal]
    fn disconnect();
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

        let worlds_manager = WorldsManager::create(self.base.to_gd().clone());
        self.worlds_manager.init(Rc::new(RefCell::new(worlds_manager)));

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

            let wm = self.worlds_manager.clone();
            let worlds_manager = wm.borrow();
            self.debug_info.bind_mut().update_debug(&worlds_manager, network_info);
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
