use common::chunks::block_position::{BlockPosition, BlockPositionTrait};
use godot::{
    engine::{Engine, InputEvent, RichTextLabel},
    prelude::*,
};

use crate::main_scene::Main;

const CAMERA_PATH: &str = "Camera";
const CAMERA_TEXT_PATH: &str = "Camera/DebugText";

macro_rules! debug_string {
    () => {
        "FPS: {:.0}
Camera position: {:.2} {:.2} {:.2}
Chunk postition: {:?}
Threads count: {}
World: {}"
    };
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct PlayerController {
    #[base]
    base: Base<Node>,
    camera: Option<Gd<Camera3D>>,
    debug_text: Option<Gd<RichTextLabel>>,
    buffer_position: Vector3,
    main: Option<Gd<Main>>,
}

#[godot_api]
impl PlayerController {
    #[signal]
    fn submit_camera_move();
}

#[godot_api]
impl NodeVirtual for PlayerController {
    fn init(base: Base<Node>) -> Self {
        PlayerController {
            base,
            camera: None,
            debug_text: None,
            main: None,
            buffer_position: Vector3::ZERO,
        }
    }

    fn ready(&mut self) {
        match self.base.try_get_node_as::<Camera3D>(CAMERA_PATH) {
            Some(c) => {
                self.camera = Some(c);
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

        self.main = Some(self.base.get_parent().unwrap().cast::<Main>());
    }

    #[allow(unused_variables)]
    fn input(&mut self, event: Gd<InputEvent>) {}

    #[allow(unused_variables)]
    fn process(&mut self, delta: f64) {
        if Engine::singleton().is_editor_hint() {
            return;
        }

        if self.camera.is_none() {
            return;
        }

        let camera = self.camera.as_deref_mut().unwrap();

        let world_str = match self.main.as_ref().unwrap().bind().world_manager.get_world() {
            Some(w) => format!("{}; chunks count: {}", w.bind().get_slug(), w.bind().get_chunks_count()),
            None => "none".to_string(),
        };

        let camera_pos = camera.get_position();
        let text = format!(
            debug_string!(),
            Engine::singleton().get_frames_per_second(),
            camera_pos.x,
            camera_pos.y,
            camera_pos.z,
            BlockPosition::new(camera_pos.x as i64, camera_pos.y as i64, camera_pos.z as i64).get_chunk_position(),
            rayon::current_num_threads(),
            world_str,
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
