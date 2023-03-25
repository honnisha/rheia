use godot::engine::RichTextLabel;
use godot::prelude::*;

use crate::client_scripts::scripts_manager::ScriptsManager;
use crate::console_handler::Console;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    #[base]
    base: Base<Node>,
    scripts_manager: ScriptsManager,
    camera: Option<Gd<Camera3D>>,
    debug_text: Option<Gd<RichTextLabel>>,
    console: Option<Gd<Console>>,
}

#[godot_api]
impl Main {}

#[godot_api]
impl NodeVirtual for Main {
    fn init(base: Base<Node>) -> Self {
        Main {
            base,
            scripts_manager: ScriptsManager::new(),
            camera: None,
            debug_text: None,
            console: None,
        }
    }

    fn ready(&mut self) {
        self.camera = Some(self.base.get_node_as("Camera"));
        self.debug_text = Some(self.base.get_node_as("Camera/DebugText"));

        match self.base.try_get_node_as("GUIControl/MarginContainer/ConsoleContainer") {
            Some(c) => self.console = Some(c),
            _ => godot_error!("Console element not found")
        }

        let camera = self.camera.as_deref_mut().unwrap();
        camera.set_position(Vector3::new(0.0, 15.0, 0.0));

        self.scripts_manager.rescan_scripts();
        godot_print!("Main scene loaded;");
    }

    #[allow(unused_variables)]
    fn process(&mut self, delta: f64) {
        let camera = self.camera.as_deref_mut().unwrap();

        let camera_pos = camera.get_position();
        let text = format!(
            "Camera position: {:.2} {:.2} {:.2}",
            camera_pos.x, camera_pos.y, camera_pos.z
        );
        self.debug_text
            .as_deref_mut()
            .unwrap()
            .set_text(GodotString::from(text));
    }
}
