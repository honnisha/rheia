use bevy::prelude::Entity;
use common::blocks::chunk_collider_info::ChunkColliderInfo;
use ndshape::{ConstShape, ConstShape3u32};

//pub type ChunkShape = ConstShape3u32<16, 16, 16>;
pub type ChunkBordersShape = ConstShape3u32<18, 18, 18>;

//pub type ChunkData = [BlockInfo; ChunkShape::SIZE as usize];
pub type ChunkColliderDataBordered = [ChunkColliderInfo; ChunkBordersShape::SIZE as usize];

pub struct ChunkSection {
    entity: Entity,
}

impl ChunkSection {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}
