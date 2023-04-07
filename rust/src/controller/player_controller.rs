use godot::{
    engine::{Engine, RichTextLabel},
    prelude::*,
};

use crate::world::chunks::chunk_info::ChunkInfo;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct PlayerController {
    #[base]
    base: Base<Node>,
    camera: Option<Gd<Camera3D>>,
    debug_text: Option<Gd<RichTextLabel>>,
    buffer_position: Vector3,
}

#[godot_api]
impl PlayerController {
    #[signal]
    fn submit_camera_move();
}

const CAMERA_PATH: &str = "Camera";
const CAMERA_TEXT_PATH: &str = "Camera/DebugText";

#[godot_api]
impl NodeVirtual for PlayerController {
    fn init(base: Base<Node>) -> Self {
        PlayerController {
            base,
            camera: None,
            debug_text: None,
            buffer_position: Vector3::ZERO,
        }
    }

    fn ready(&mut self) {
        match self.base.try_get_node_as::<Camera3D>(CAMERA_PATH) {
            Some(c) => {
                self.camera = Some(c);

                let camera = self.camera.as_deref_mut().unwrap();
                camera.set_position(Vector3::new(0.0, 40.0, 0.0));
            }
            None => {
                godot_error!("Camera element not found for PlayerController");
            }
        }

        match self.base.try_get_node_as::<RichTextLabel>(CAMERA_TEXT_PATH) {
            Some(c) => {
                self.debug_text = Some(c);
            }
            None => {
                godot_error!("Debug text element not found for PlayerController");
            }
        }
    }

    #[allow(unused_variables)]
    fn process(&mut self, delta: f64) {
        if Engine::singleton().is_editor_hint() {
            return;
        }

        if self.camera.is_none() {
            return;
        }

        let camera = self.camera.as_deref_mut().unwrap();

        let camera_pos = camera.get_global_position();
        let text = format!(
            "FPS: {:.0}\nCamera position: [b]{:.2} {:.2} {:.2}[/b]\nChunk postition: [b]{:?}[/b]",
            Engine::singleton().get_frames_per_second(),
            camera_pos.x,
            camera_pos.y,
            camera_pos.z,
            ChunkInfo::get_chunk_pos_by_global(&[camera_pos.x as i32, camera_pos.y as i32, camera_pos.z as i32])
        );
        self.debug_text
            .as_deref_mut()
            .unwrap()
            .set_text(GodotString::from(text));

        if self.buffer_position.distance_to(camera_pos) > 0.1 {
            self.buffer_position = camera_pos;
            self.base
                .emit_signal("submit_camera_move".into(), &[camera_pos.to_variant()]);
        }
    }
}
