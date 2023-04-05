use crate::{
    controller::player_controller::PlayerController,
    utils::schematics::{convert_schem_to_blockinfo, load_schem_data},
    world::chunks::chunks_manager::ChunksManager,
};
use godot::{engine::Engine, prelude::*};
use std::{collections::HashMap, env};

use self::{blocks::blocks_storage::BlockType, chunks::block_info::BlockInfo};

pub mod blocks;
pub mod chunks;
pub mod world_generator;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct World {
    #[base]
    base: Base<Node>,
    chunks_manager: Option<Gd<ChunksManager>>,
}

#[godot_api]
impl World {
    #[func]
    fn handle_camera_move(&mut self, global_position: Vector3) {
        if let Some(manager) = self.chunks_manager.as_mut() {
            manager
                .bind_mut()
                .update_camera_position(&mut self.base, global_position);
        }
    }

    pub fn modify_block(&mut self, pos: &[i32; 3], block_info: BlockInfo) {
        self.chunks_manager
            .as_mut()
            .unwrap()
            .bind_mut()
            .modify_block(pos, block_info)
    }

    pub fn modify_block_batch(&mut self, data: HashMap<[i32; 3], BlockInfo>) -> i32 {
        self.chunks_manager
            .as_mut()
            .unwrap()
            .bind_mut()
            .modify_block_batch(data)
    }
}

#[godot_api]
impl NodeVirtual for World {
    fn init(base: Base<Node>) -> Self {
        World {
            base,
            chunks_manager: None,
        }
    }

    fn ready(&mut self) {
        if Engine::singleton().is_editor_hint() {
            return;
        }

        self.chunks_manager = self.base.try_get_node_as("ChunksManager");
        if self.chunks_manager.is_some() {
            let player_controller = self
                .base
                .get_parent()
                .unwrap()
                .try_get_node_as::<PlayerController>("PlayerController");
            if player_controller.is_some() {
                player_controller.unwrap().bind_mut().connect(
                    "submit_camera_move".into(),
                    Callable::from_object_method(self.base.share(), "handle_camera_move"),
                    0,
                );
            } else {
                godot_error!("PlayerController element not found for World");
            }
        } else {
            godot_error!("ChunksManager element not found for World");
        }

        godot_print!("World loaded;");
    }

    #[allow(unused_variables)]
    fn process(&mut self, delta: f64) {
        if Engine::singleton().is_editor_hint() {
            return;
        }

        let input = Input::singleton();
        if input.is_action_just_pressed("ui_up".into(), false) {
            self.modify_block(&[0_i32, 20_i32, -20_i32], BlockInfo::new(BlockType::CraftingTable));
            godot_print!("block changed;");
        }
        if input.is_action_just_pressed("ui_right".into(), false) {
            let mut path = env::current_dir().unwrap().clone();
            path.pop();
            path.push("schematics");
            path.push("arena.schem");
            let schem = match load_schem_data(&path) {
                Ok(e) => e,
                Err(e) => {
                    godot_print!("Schem not found: {:?} error: {:?}", path, e);
                    return;
                }
            };

            let modify_data = convert_schem_to_blockinfo(&[0_i32, 40_i32, 0_i32], &schem);
            let source_len = modify_data.len();
            let count = self.modify_block_batch(modify_data);
            println!(
                "Offset {:?} width:{:?} height:{:?} length:{:?}",
                schem.get_local_offset(), schem.width, schem.height, schem.length
            );
            println!(
                "Schem {:?} pasted; blocks size: {}; modified blocks: {}",
                path, source_len, count
            );
        }
    }
}
