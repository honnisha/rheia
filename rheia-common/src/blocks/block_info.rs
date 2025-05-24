use serde::{Deserialize, Serialize};

use crate::chunks::rotation::Rotation;

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

pub type BlockIndexType = u16;

#[derive(Clone, Copy, Eq, Serialize, Deserialize)]
pub struct BlockInfo {
    id: BlockIndexType,
    face: Option<BlockFace>,
}

impl std::fmt::Debug for BlockInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let face = match self.face {
            Some(f) => format!(" face:{:?}", f),
            None => "".to_string(),
        };
        write!(f, "b#{}{}", self.id, face)
    }
}

impl BlockInfo {
    pub fn create(id: BlockIndexType, face: Option<BlockFace>) -> BlockInfo {
        BlockInfo { id, face }
    }

    pub fn get_id(&self) -> BlockIndexType {
        self.id
    }

    pub fn get_face(&self) -> Option<&BlockFace> {
        self.face.as_ref()
    }

    pub fn set_face(&mut self, face: Option<BlockFace>) {
        self.face = face;
    }
}

impl PartialEq for BlockInfo {
    fn eq(&self, other: &BlockInfo) -> bool {
        self.id == other.id && self.face == other.face
    }
}
