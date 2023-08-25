use bevy::prelude::Entity;
use common::blocks::blocks_storage::BlockType;
use ndshape::{ConstShape, ConstShape3u32};

//pub type ChunkShape = ConstShape3u32<16, 16, 16>;
pub type ChunkBordersShape = ConstShape3u32<18, 18, 18>;

//pub type ChunkData = [BlockInfo; ChunkShape::SIZE as usize];
pub type ChunkDataBordered = [BlockType; ChunkBordersShape::SIZE as usize];

pub struct ChunkSection {
    entity: Entity,
}

impl ChunkSection {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}
