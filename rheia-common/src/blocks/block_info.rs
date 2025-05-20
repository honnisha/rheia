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
}

impl PartialEq for BlockInfo {
    fn eq(&self, other: &BlockInfo) -> bool {
        self.id == other.id && self.face == other.face
    }
}
