use serde::{Deserialize, Serialize};

use crate::chunks::{chunk_data::BlockIndexType, rotation::Rotation};

use super::block_type::BlockType;

#[derive(Debug, Clone, Copy, Eq, Serialize, Deserialize, PartialEq)]
pub enum BlockFace {
    East,
    North,
    South,
    West,
}

impl Default for BlockFace {
    fn default() -> Self {
        BlockFace::South
    }
}

impl BlockFace {
    pub fn rotate_left(&self) -> BlockFace {
        match *self {
            BlockFace::East => BlockFace::South,
            BlockFace::North => BlockFace::East,
            BlockFace::South => BlockFace::West,
            BlockFace::West => BlockFace::North,
        }
    }

    pub fn rotate_right(&self) -> BlockFace {
        match *self {
            BlockFace::East => BlockFace::North,
            BlockFace::North => BlockFace::West,
            BlockFace::South => BlockFace::East,
            BlockFace::West => BlockFace::South,
        }
    }

    pub fn get_rotation(&self) -> Rotation {
        match *self {
            BlockFace::East => Rotation::new(0.0, 270.0),
            BlockFace::North => Rotation::new(0.0, 180.0),
            BlockFace::South => Rotation::new(0.0, 0.0),
            BlockFace::West => Rotation::new(0.0, 90.0),
        }
    }
}

pub fn generate_block_id(_block_type: &BlockType, last_id: BlockIndexType) -> BlockIndexType {
    last_id + 1
}
