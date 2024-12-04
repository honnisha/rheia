use common::{
    blocks::block_type::{BlockContent, BlockType},
    chunks::block_position::ChunkBlockPosition,
};
use godot::prelude::*;
use network::messages::ChunkDataType;

use crate::world::block_storage::BlockStorage;

/// Container for custom objects of map per chunk section
#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct ObjectsContainer {
    pub(crate) base: Base<Node3D>,
}

impl ObjectsContainer {
    pub fn setup(&mut self, chunk_data: &ChunkDataType, block_storage: &BlockStorage) {
        for (chunk_block_position, block_info) in chunk_data.iter() {
            let Some(block_type) = block_storage.get(&block_info.get_id()) else {
                continue;
            };

            match block_type.get_block_content() {
                BlockContent::ModelCube { model } => {
                    log::info!(
                        target: "container",
                        "block init chunk_block_position:{:?} model:{}",
                        chunk_block_position,
                        model
                    );
                }
                _ => continue,
            }
        }
    }

    pub fn remove(&mut self, chunk_block_position: ChunkBlockPosition) {
        log::info!(target: "container", "block remove chunk_block_position:{:?}", chunk_block_position);
    }

    pub fn update_block_model(&mut self, chunk_block_position: ChunkBlockPosition, block_type: &BlockType) {
        let model = match block_type.get_block_content() {
            BlockContent::ModelCube { model } => model,
            _ => panic!("update_block_model called for non model"),
        };
        log::info!(
            target: "container",
            "block update chunk_block_position:{:?} model:{}",
            chunk_block_position,
            model
        );
    }
}
