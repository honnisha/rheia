use godot::engine::packed_scene::GenEditState;
use godot::engine::RichTextLabel;
use godot::prelude::*;

use crate::client_scripts::scripts_manager::ScriptsManager;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    #[base]
    base: Base<Node>,
    scripts_manager: ScriptsManager,
    camera: Option<Gd<Camera3D>>,
    debug_text: Option<Gd<RichTextLabel>>,
}

#[godot_api]
impl Main {
}

#[godot_api]
impl GodotExt for Main {
    fn init(base: Base<Node>) -> Self {
        Main {
            base,
            scripts_manager: ScriptsManager::new(),
            camera: None,
            debug_text: None,
        }
    }

    fn ready(&mut self) {
        self.camera = Some(self.base.get_node_as("Camera"));
        self.debug_text = Some(self.base.get_node_as("Camera/DebugText"));

        let camera = self.camera.as_deref_mut().unwrap();
        camera.set_position(Vector3::new(0.0, 0.0, 5.0));

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
