use common::chunks::block_position::{BlockPosition, BlockPositionTrait};
use godot::{
    engine::{Engine, MarginContainer, RichTextLabel},
    prelude::*,
};
use log::error;

use crate::world::world_manager::WorldManager;

const TEXT_FIRST_PATH: &str =
    "MarginContainer/VBoxContainer/HBoxContainer/PanelContainer/MarginContainer/RichTextLabel";

macro_rules! debug_string {
    () => {
        "FPS: {:.0}
Camera position: {:.2} {:.2} {:.2}
Chunk postition: {}
Threads count: {}
World: {}"
    };
}

#[derive(GodotClass)]
#[class(base=MarginContainer)]
pub struct DebugInfo {
    #[base]
    base: Base<MarginContainer>,
    first_text: Option<Gd<RichTextLabel>>,
}

impl DebugInfo {
    pub fn update_debug(&mut self, world_manager: &WorldManager, camera: &mut Camera3D) {
        let world_str = match world_manager.get_world() {
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
        self.first_text
            .as_deref_mut()
            .unwrap()
            .set_text(GodotString::from(text));
    }
}

#[godot_api]
impl NodeVirtual for DebugInfo {
    fn init(base: Base<MarginContainer>) -> Self {
        Self {
            base: base,
            first_text: None,
        }
    }

    fn ready(&mut self) {
        if Engine::singleton().is_editor_hint() {
            return;
        }

        match self.base.try_get_node_as::<RichTextLabel>(TEXT_FIRST_PATH) {
            Some(c) => {
                self.first_text = Some(c);
            }
            None => {
                error!("TEXT_FIRST_PATH not found");
            }
        }
    }
}
