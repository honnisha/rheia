use serde::{Deserialize, Serialize};

use crate::utils::fix_chunk_loc_pos;

use super::chunk_position::ChunkPosition;

pub trait BlockPositionTrait {
    fn get_chunk_position(&self) -> ChunkPosition;
}

/// Uses as block position inside chunk section CHUNK_SIZExCHUNK_SIZE
#[derive(Clone, Copy, Default, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct ChunkBlockPosition {
    x: u8,
    y: u8,
    z: u8,
}

impl std::fmt::Debug for ChunkBlockPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

impl ChunkBlockPosition {
    pub fn new(x: u8, y: u8, z: u8) -> Self {
        Self { x, y, z }
    }
}

/// Global block position
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct BlockPosition {
    x: i64,
    y: i64,
    z: i64,
}

impl BlockPosition {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}

impl BlockPositionTrait for BlockPosition {
    fn get_chunk_position(&self) -> ChunkPosition {
        ChunkPosition::new(fix_chunk_loc_pos(self.x as i64), fix_chunk_loc_pos(self.z as i64))
    }
}
