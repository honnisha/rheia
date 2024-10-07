use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, Serialize, Deserialize, PartialEq)]
pub enum BlockFace {
    Down,
    East,
    North,
    South,
    Up,
    West,
}

#[derive(Debug, Clone, Copy, Eq, Serialize, Deserialize)]
pub struct BlockInfo {
    id: u32,
    face: Option<BlockFace>,
}

impl BlockInfo {
    pub fn create(id: u32, face: Option<BlockFace>) -> BlockInfo {
        BlockInfo { id, face }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

impl PartialEq for BlockInfo {
    fn eq(&self, other: &BlockInfo) -> bool {
        self.id == other.id && self.face == other.face
    }
}
