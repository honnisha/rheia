use crate::client_scripts::resource_manager::ScriptsManager;
use crate::console::console_handler::Console;
use crate::network::client::NetworkClient;
use godot::engine::Engine;
use godot::prelude::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    #[base]
    base: Base<Node>,
    resource_manager: ScriptsManager,
    client: Option<NetworkClient>,
}

#[godot_api]
impl Main {
    //self.scripts_manager.run_event(
    //    "onConsoleCommand".to_string(),
    //    vec![Dynamic::from(new_text.to_string())],
    //);

    fn handle_console_command(&mut self, command: String) {
        if command.len() == 0 {
            return;
        }
        if let Some(client) = self.client.as_mut() {
            client.send_console_command(command);
        }
    }
}

#[godot_api]
impl NodeVirtual for Main {
    fn init(base: Base<Node>) -> Self {
        Main {
            base,
            resource_manager: ScriptsManager::new(),
            client: None,
        }
    }

    fn ready(&mut self) {
        godot_print!("loading HonnyCraft version: {}", VERSION);

        if Engine::singleton().is_editor_hint() {
            return;
        }

        self.client = Some(NetworkClient::init(
            "127.0.0.1:14191".to_string(),
            "TestUser".to_string(),
        ));
    }

    fn process(&mut self, delta: f64) {
        for message in Console::get_input_receiver().try_iter() {
            self.handle_console_command(message);
        }

        if let Some(client) = self.client.as_mut() {
            client.update(delta);
        }
    }

    fn exit_tree(&mut self) {
        if let Some(client) = self.client.as_mut() {
            client.disconnect();
        }
    }
}
