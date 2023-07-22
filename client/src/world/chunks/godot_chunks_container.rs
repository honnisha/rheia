use ahash::AHashMap;
use common::{blocks::block_info::BlockInfo, chunks::{chunk_position::ChunkPosition, block_position::BlockPosition}};
use godot::prelude::*;

use super::godot_chunk_column::ChunkColumn;

/// Contains all chunk sections
#[derive(GodotClass)]
#[class(base=Node)]
pub struct ChunksContainer {
    #[base]
    base: Base<Node>,
    chunks: AHashMap<ChunkPosition, Gd<ChunkColumn>>,
}

impl ChunksContainer {
    pub fn create(base: Base<Node>) -> Self {
        Self {
            base,
            chunks: Default::default(),
        }
    }

    pub fn modify_block(&self, global_pos: &BlockPosition, block_info: BlockInfo) {
        todo!();
    }
}

#[godot_api]
impl ChunksContainer {
    /// For default godot init; only World::create is using
    fn init(base: Base<Node>) -> Self {
        Self::create(base)
    }

    fn ready(&mut self) {}
}
