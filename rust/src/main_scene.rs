use crate::client_scripts::scripts_manager::ScriptsManager;
use crate::console_handler::Console;
use crate::console_handler::CONSOLE_CHANNEL;
use godot::engine::Engine;
use godot::prelude::*;
use rhai::Dynamic;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    #[base]
    base: Base<Node>,
    scripts_manager: ScriptsManager,
}

#[godot_api]
impl Main {
    fn handle_console_command(&mut self, new_text: String) {
        godot_print!("console_command: {}", new_text);
        self.scripts_manager.run_event(
            "onConsoleCommand".to_string(),
            vec![Dynamic::from(new_text.to_string())],
        );
    }
}

pub const CONSOLE_PATH: &str = "GUIControl/MarginContainer/ConsoleContainer";

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

        self.scripts_manager.rescan_scripts(&self.base);
        godot_print!("Main scene loaded;");
    }

    #[allow(unused_variables)]
    fn process(&mut self, delta: f64) {
        for message in CONSOLE_CHANNEL.1.try_iter() {
            self.handle_console_command(message);
        }
    }
}
