use crate::{utils::fix_chunk_loc_pos, CHUNK_SIZE};
use serde::{Deserialize, Serialize};

use super::{chunk_position::ChunkPosition, position::Vector3};

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
#[derive(Clone, Copy, Default, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct BlockPosition {
    x: i64,
    y: i64,
    z: i64,
}

impl BlockPosition {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    pub fn from_global(pos: &Vector3) -> Self {
        Self {
            x: pos.x as i64,
            y: pos.y as i64,
            z: pos.z as i64,
        }
    }

    pub fn get_block_position(&self) -> (u32, ChunkBlockPosition) {
        let size = CHUNK_SIZE as f32;
        let section = (self.y as f32 / size).floor() as u32;
        let block_position = ChunkBlockPosition::new(
            (self.x as f32 % size) as u8,
            (self.y as f32 % size) as u8,
            (self.z as f32 % size) as u8,
        );
        return (section, block_position);
    }
}

impl std::fmt::Debug for BlockPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

impl BlockPositionTrait for BlockPosition {
    fn get_chunk_position(&self) -> ChunkPosition {
        ChunkPosition::new(fix_chunk_loc_pos(self.x as i64), fix_chunk_loc_pos(self.z as i64))
    }
}
