use ahash::AHashMap;
use common::blocks::block_info::BlockInfo;

pub type ChunkPositionType = [u8; 3];
pub type ChunkDataType = AHashMap<ChunkPositionType, BlockInfo>;

pub struct ChunkSection {
    pub(crate) chunk_data: ChunkDataType,
}

impl ChunkSection {
    pub fn new() -> Self {
        Self {
            chunk_data: AHashMap::new(),
        }
    }
}
