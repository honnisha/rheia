use common::chunks::block_position::{BlockPosition, BlockPositionTrait};
use godot::{
    engine::{Engine, MarginContainer, RichTextLabel},
    prelude::*,
};
use log::error;

use crate::world::world_manager::WorldManager;

const TEXT_FIRST_PATH: &str = "MarginContainer/VBoxContainer/Row1/PanelContainer/MarginContainer/RichTextLabel";

macro_rules! debug_first_string {
    () => {
        "FPS: {:.0}
Camera position: {:.2} {:.2} {:.2}
Threads count: {}"
    };
}

const TEXT_SECOND_PATH: &str = "MarginContainer/VBoxContainer/Row2/PanelContainer/MarginContainer/RichTextLabel";
macro_rules! debug_second_string {
    () => {
        "World: {}
Chunks loaded: {}
Chunk position: {}
Chunk info: {}"
    };
}

#[derive(GodotClass)]
#[class(base=MarginContainer)]
pub struct DebugInfo {
    #[base]
    base: Base<MarginContainer>,
    first_text: Option<Gd<RichTextLabel>>,
    second_text: Option<Gd<RichTextLabel>>,
}

impl DebugInfo {
    pub fn update_debug(&mut self, world_manager: &WorldManager, camera: &mut Camera3D) {
        let camera_pos = camera.get_position();

        let first_text = format!(
            debug_first_string!(),
            Engine::singleton().get_frames_per_second(),
            camera_pos.x,
            camera_pos.y,
            camera_pos.z,
            rayon::current_num_threads()
        );
        self.first_text
            .as_deref_mut()
            .unwrap()
            .set_text(GodotString::from(first_text));

        let chunk_pos =
            BlockPosition::new(camera_pos.x as i64, camera_pos.y as i64, camera_pos.z as i64).get_chunk_position();
        let second_text = match world_manager.get_world() {
            Some(w) => {
                let world = w.bind();
                let chunk_info = match world.get_chunk(&chunk_pos) {
                    Some(c) => {
                        let c = c.borrow();
                        let chunk_column = c.get_chunk_column().bind();
                        format!("sended:{} loaded:{}", chunk_column.is_sended(), chunk_column.is_loaded())
                    },
                    None => "-".to_string(),
                };
                format!(
                    debug_second_string!(),
                    world.get_slug(),
                    world.get_chunks_count(),
                    chunk_pos,
                    chunk_info,
                )
            }
            None => "World: -".to_string(),
        };
        self.second_text
            .as_deref_mut()
            .unwrap()
            .set_text(GodotString::from(second_text));
    }
}

#[godot_api]
impl NodeVirtual for DebugInfo {
    fn init(base: Base<MarginContainer>) -> Self {
        Self {
            base: base,
            first_text: Default::default(),
            second_text: Default::default(),
        }
    }

    fn ready(&mut self) {
        if Engine::singleton().is_editor_hint() {
            return;
        }

        if let Some(c) = self.base.try_get_node_as::<RichTextLabel>(TEXT_FIRST_PATH) {
            self.first_text = Some(c);
        } else {
            error!("TEXT_FIRST_PATH not found");
        }

        if let Some(c) = self.base.try_get_node_as::<RichTextLabel>(TEXT_SECOND_PATH) {
            self.second_text = Some(c);
        } else {
            error!("TEXT_SECOND_PATH not found");
        }
    }
}
