use crate::world::chunks::chunks_manager::ChunksManager;
use common::blocks::block_info::BlockInfo;
use godot::{
    engine::{node::InternalMode},
    prelude::*,
};
use std::{collections::HashMap};

pub mod chunks;
pub mod world_generator;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct World {
    #[base]
    base: Base<Node>,
    slug: String,
    chunks_manager: Option<Gd<ChunksManager>>,
}

#[godot_api]
impl World {
    #[func]
    fn handle_camera_move(&mut self, global_position: Vector3) {
        self.chunks_manager
            .as_mut()
            .unwrap()
            .bind_mut()
            .update_camera_position(&mut self.base, global_position);
    }

    pub fn modify_block(&mut self, pos: &[i32; 3], block_info: BlockInfo) {
        self.chunks_manager
            .as_mut()
            .unwrap()
            .bind_mut()
            .modify_block(pos, block_info)
    }

    pub fn modify_block_batch(&mut self, data: HashMap<[i32; 3], HashMap<u32, BlockInfo>>) {
        self.chunks_manager
            .as_mut()
            .unwrap()
            .bind_mut()
            .modify_block_batch(data)
    }
}

impl World {
    pub fn create(base: Base<Node>, slug: String) -> Self {
        World {
            base,
            slug: slug,
            chunks_manager: None,
        }
    }

    pub fn get_slug(&self) -> &String {
        &self.slug
    }
}

#[godot_api]
impl NodeVirtual for World {
    fn init(base: Base<Node>) -> Self {
        World::create(base, "Godot".to_string())
    }

    fn ready(&mut self) {
        let mut chunks_manager = Gd::<ChunksManager>::with_base(|base| ChunksManager::create(base));

        let chunks_manager_name = GodotString::from("ChunksManager");
        chunks_manager.bind_mut().set_name(chunks_manager_name.clone());

        self.base
            .add_child(chunks_manager.upcast(), true, InternalMode::INTERNAL_MODE_FRONT);
        self.chunks_manager = Some(self.base.get_node_as::<ChunksManager>(chunks_manager_name));
    }
}
