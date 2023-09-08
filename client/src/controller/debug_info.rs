use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

use common::chunks::block_position::{BlockPosition, BlockPositionTrait};
use godot::{
    engine::{Engine, MarginContainer, RichTextLabel},
    prelude::*,
};
use lazy_static::lazy_static;
use log::error;

use crate::{world::world_manager::WorldManager};


lazy_static! {
    static ref DEBUG_ACTIVE: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
}

const TEXT_FIRST_PATH: &str = "MarginContainer/VBoxContainer/Row1/PanelContainer/MarginContainer/RichTextLabel";

macro_rules! debug_first_string {
    () => {
        "FPS: {:.0}
Controller position: {}
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
    pub fn is_active() -> bool {
        DEBUG_ACTIVE.load(Ordering::Relaxed)
    }

    pub fn toggle(&mut self, state: bool) {
        DEBUG_ACTIVE.store(state, Ordering::Relaxed);

        self.base.set_visible(DebugInfo::is_active());
    }

    pub fn update_debug(
        &mut self,
        world_manager: GdRef<WorldManager>,
        camera: &Gd<Camera3D>,
    ) {
        if !DebugInfo::is_active() {
            return;
        }

        let controller_positioin = match world_manager.get_player_controller().bind().get_handler() {
            Some(h) => {
                let controller_pos = h.get_position(&camera);
                format!(
                    "{:.2} {:.2} {:.2} yaw:{:.2} pitch:{:.2}",
                    controller_pos.x,
                    controller_pos.y,
                    controller_pos.z,
                    h.get_yaw(&camera),
                    h.get_pitch(&camera),
                )
            },
            None => "-".to_string(),
        };

        let first_text = format!(
            debug_first_string!(),
            Engine::singleton().get_frames_per_second(),
            controller_positioin,
            rayon::current_num_threads()
        );
        self.first_text
            .as_deref_mut()
            .unwrap()
            .set_text(GodotString::from(first_text));

        let camera_pos = camera.get_position();
        let chunk_pos =
            BlockPosition::new(camera_pos.x as i64, camera_pos.y as i64, camera_pos.z as i64).get_chunk_position();
        let second_text = match world_manager.get_world() {
            Some(w) => {
                let world = w.bind();
                let chunk_info = match world.get_chunk(&chunk_pos) {
                    Some(c) => {
                        let c = c.borrow();
                        format!(
                            "sended:{} loaded:{}",
                            c.is_sended(),
                            c.is_loaded()
                        )
                    }
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
        self.base.set_visible(false);

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
