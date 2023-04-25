use std::sync::{Arc, Mutex, MutexGuard};

use crate::client_scripts::resource_manager::ResourceManager;
use crate::console::console_handler::Console;
use crate::network::client::NetworkClient;
use godot::engine::Engine;
use godot::prelude::*;
use lazy_static::lazy_static;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    #[base]
    base: Base<Node>,
    resource_manager: ResourceManager,
}

lazy_static! {
    static ref NETWORK_CLIENT: Arc<Mutex<NetworkClient>> = Arc::new(Mutex::new(NetworkClient::init()));
}

#[godot_api]
impl Main {
    fn handle_console_command(&mut self, command: String) {
        if command.len() == 0 {
            return;
        }
        Main::get_client().send_console_command(command);
    }

    pub fn get_client() -> MutexGuard<'static, NetworkClient> {
        NETWORK_CLIENT.lock().unwrap()
    }
}

#[godot_api]
impl NodeVirtual for Main {
    fn init(base: Base<Node>) -> Self {
        Main {
            base,
            resource_manager: ResourceManager::new(),
        }
    }

    fn ready(&mut self) {
        godot_print!("Loading HonnyCraft version: {}", VERSION);

        if Engine::singleton().is_editor_hint() {
            return;
        }

        NETWORK_CLIENT.lock().unwrap().create_client(
            "127.0.0.1:14191".to_string(),
            "TestUser".to_string(),
        );
    }

    fn process(&mut self, delta: f64) {
        for message in Console::get_input_receiver().try_iter() {
            self.handle_console_command(message);
        }

        Main::get_client().update(delta, &mut self.resource_manager);
    }

    fn exit_tree(&mut self) {
        Main::get_client().disconnect();
    }
}
