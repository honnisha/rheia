use godot::engine::Engine;
use godot::prelude::*;
use rhai::Dynamic;

use crate::client_scripts::scripts_manager::ScriptsManager;
use crate::console_handler::Console;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    #[base]
    base: Base<Node>,
    scripts_manager: ScriptsManager,
}

#[godot_api]
impl Main {
    #[func]
    fn handle_console_command(&mut self, new_text: GodotString) {
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
        if Engine::singleton().is_editor_hint() {
            return;
        }

        let console = self.base.try_get_node_as::<Console>(CONSOLE_PATH);
        if console.is_some() {
            console.unwrap().bind_mut().connect(
                "submit_console_command".into(),
                Callable::from_object_method(self.base.share(), "handle_console_command"),
                0,
            );
        } else {
            godot_error!("Console element not found");
        }

        self.scripts_manager.rescan_scripts(&self.base);
        godot_print!("Main scene loaded;");
    }
}
