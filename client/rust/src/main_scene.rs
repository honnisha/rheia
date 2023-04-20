use crate::client_scripts::scripts_manager::ScriptsManager;
use crate::console::console_handler::{Console};
use crate::network::client::NetworkClient;
use godot::engine::Engine;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    #[base]
    base: Base<Node>,
    scripts_manager: ScriptsManager,
    client: NetworkClient,
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
        let finded = self.scripts_manager.run_command(command.clone());
        if !finded {
            Console::send_message(format!("[color=#DE4747]Command \"{}\" not found[/color]", command));
        }
    }
}

#[godot_api]
impl NodeVirtual for Main {
    fn init(base: Base<Node>) -> Self {
        let client = NetworkClient::init("127.0.0.1:14191".to_string());
        Main {
            base,
            scripts_manager: ScriptsManager::new(),
            client: client,
        }
    }

    fn ready(&mut self) {
        godot_print!("Start loading main scene;");
        if Engine::singleton().is_editor_hint() {
            return;
        }

        self.scripts_manager.rescan_scripts();
        godot_print!("Main scene loaded;");
    }

    fn process(&mut self, delta: f64) {
        for message in Console::get_input_receiver().try_iter() {
            self.handle_console_command(message);
        }

        self.client.update(delta);
    }

    fn exit_tree(&mut self) {
        self.client.disconnect();
    }
}
