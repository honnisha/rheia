use godot::engine::packed_scene::GenEditState;
use godot::engine::RichTextLabel;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    #[base]
    base: Base<Node>,
    camera: Option<Gd<Camera3D>>,
    debug_text: Option<Gd<RichTextLabel>>,
}

#[godot_api]
impl Main {
    #[func]
    pub fn new_game(&mut self) {
        godot_print!("New game");
    }
}

#[godot_api]
impl GodotExt for Main {
    fn init(base: Base<Node>) -> Self {
        Main {
            base,
            camera: None,
            debug_text: None,
        }
    }

    fn ready(&mut self) {
        self.camera = Some(self.base.get_node_as("Camera"));
        self.debug_text = Some(self.base.get_node_as("Camera/DebugText"));

        let camera = self.camera.as_deref_mut().unwrap();
        camera.set_position(Vector3::new(10.0, 0.0, 10.0));

        godot_print!("Ready");
    }

    #[allow(unused_variables)]
    fn process(&mut self, delta: f64) {
        let camera = self.camera.as_deref_mut().unwrap();

        let camera_pos = camera.get_position();
        let text = format!(
            "Camera position: {} {} {}",
            camera_pos.x, camera_pos.y, camera_pos.z
        );
        self.debug_text
            .as_deref_mut()
            .unwrap()
            .set_text(GodotString::from(text));
    }
}

#[allow(dead_code)]
fn instantiate_scene<Root>(scene: &PackedScene) -> Gd<Root>
where
    Root: GodotClass + Inherits<Node>,
{
    let s = scene
        .instantiate(GenEditState::GEN_EDIT_STATE_DISABLED)
        .expect("scene instantiated");

    s.cast::<Root>()
}
