use crate::{utils::fix_chunk_loc_pos, CHUNK_SIZE};
use ndshape::{AbstractShape, ConstShape3u16};
use serde::{Deserialize, Serialize};

use super::{chunk_position::ChunkPosition, position::Vector3};

pub trait BlockPositionTrait {
    fn get_chunk_position(&self) -> ChunkPosition;
}

/// Uses as block position inside chunk section CHUNK_SIZExCHUNK_SIZE
#[derive(Clone, Copy, Default, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct ChunkBlockPosition {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

impl ChunkBlockPosition {
    pub fn new(x: u8, y: u8, z: u8) -> Self {
        Self { x, y, z }
    }

    pub fn linearize(&self) -> u16 {
        let shape = ConstShape3u16::<16, 16, 16>;
        shape.linearize([self.x as u16, self.y as u16, self.z as u16])
    }

    pub fn delinearize(index: u16) -> Self {
        let shape = ConstShape3u16::<16, 16, 16>;
        let pos = shape.delinearize(index);
        Self::new(pos[0] as u8, pos[1] as u8, pos[2] as u8)
    }
}

impl std::fmt::Debug for ChunkBlockPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "[CB:{},{},{}]", self.x, self.y, self.z)
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

    pub fn from_chunk_position(chunk_position: &ChunkPosition, y: &u32, block_position: &ChunkBlockPosition) -> Self {
        Self {
            x: (chunk_position.x as f32 * CHUNK_SIZE as f32) as i64 + block_position.x as i64,
            y: (*y as f32 * CHUNK_SIZE as f32) as i64 + block_position.y as i64,
            z: (chunk_position.z as f32 * CHUNK_SIZE as f32) as i64 + block_position.z as i64,
        }
    }

    pub fn from_position(position: &Vector3) -> Self {
        Self {
            x: position.x.floor() as i64,
            y: position.y.floor() as i64,
            z: position.z.floor() as i64,
        }
    }

    pub fn get_position(&self) -> Vector3 {
        Vector3::new(self.x as f32, self.y as f32, self.z as f32)
    }

    fn fix_chunk_negative(pos: i64) -> u8 {
        let mut p = pos as f32 % CHUNK_SIZE as f32;
        if p < 0.0 {
            p = CHUNK_SIZE as f32 + p;
        }
        p as u8
    }

    pub fn get_block_position(&self) -> (u32, ChunkBlockPosition) {
        let block_position = ChunkBlockPosition::new(
            BlockPosition::fix_chunk_negative(self.x),
            BlockPosition::fix_chunk_negative(self.y),
            BlockPosition::fix_chunk_negative(self.z),
        );
        let section = (self.y as f32 / CHUNK_SIZE as f32).floor() as u32;
        return (section, block_position);
    }
}

impl std::fmt::Debug for BlockPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "[P:{},{},{}]", self.x, self.y, self.z)
    }
}

impl BlockPositionTrait for BlockPosition {
    fn get_chunk_position(&self) -> ChunkPosition {
        ChunkPosition::new(fix_chunk_loc_pos(self.x as i64), fix_chunk_loc_pos(self.z as i64))
    }
}

#[cfg(test)]
mod tests {
    use super::BlockPosition;
    use crate::chunks::block_position::ChunkBlockPosition;

    #[test]
    fn test_block_position() {
        let (section, block_position) = BlockPosition::new(1, 1, 1).get_block_position();
        assert_eq!(section, 0);
        assert_eq!(block_position, ChunkBlockPosition::new(1, 1, 1));
    }

    #[test]
    fn test_block_position_negative() {
        let (section, block_position) = BlockPosition::new(-1, 1, -1).get_block_position();
        assert_eq!(section, 0);
        assert_eq!(block_position, ChunkBlockPosition::new(15, 1, 15));
    }
}
