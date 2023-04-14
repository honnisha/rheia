use crate::client_scripts::scripts_manager::ScriptsManager;
use crate::console::console_handler::{CONSOLE_OUTPUT_CHANNEL, Console};
use godot::engine::Engine;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    #[base]
    base: Base<Node>,
    scripts_manager: ScriptsManager,
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
        Main {
            base,
            scripts_manager: ScriptsManager::new(),
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

    #[allow(unused_variables)]
    fn process(&mut self, delta: f64) {
        for message in CONSOLE_OUTPUT_CHANNEL.1.try_iter() {
            self.handle_console_command(message);
        }
    }
}
