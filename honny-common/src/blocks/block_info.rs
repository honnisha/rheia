use serde::{Serialize, Deserialize};

use super::{blocks_storage::BlockType, block_type_info::BlockTypeInfo};

#[derive(Debug, Copy, Serialize, Deserialize)]
pub struct BlockInfo {
    block_type: BlockType,
}

impl BlockInfo {
    pub fn new(block_type: BlockType) -> BlockInfo {
        BlockInfo {
            block_type: block_type,
        }
    }

    #[allow(dead_code)]
    pub fn get_block_type_info(&self) -> &'static BlockTypeInfo {
        self.block_type.get_block_type_info().unwrap()
    }

    pub fn get_block_type(&self) -> BlockType {
        self.block_type
    }
}

impl Clone for BlockInfo {
    fn clone(&self) -> BlockInfo {
        BlockInfo {
            block_type: self.block_type,
        }
    }
}

impl PartialEq for BlockInfo {
    fn eq(&self, other: &BlockInfo) -> bool {
        self.block_type == other.block_type
    }
}
