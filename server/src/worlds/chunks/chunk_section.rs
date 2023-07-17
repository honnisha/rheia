use std::collections::HashMap;

use common::blocks::block_info::BlockInfo;

pub type ChunkPositionType = [u8; 3];
pub type ChunkDataType = HashMap<ChunkPositionType, BlockInfo>;

#[derive(Clone)]
pub struct ChunkSection {
    pub(crate) chunk_data: ChunkDataType,
}

impl ChunkSection {
    pub fn new() -> Self {
        Self {
            chunk_data: HashMap::new(),
        }
    }
}
