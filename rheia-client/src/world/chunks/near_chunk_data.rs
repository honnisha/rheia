use common::chunks::chunk_position::ChunkPosition;

use super::{chunk_column::ColumnDataLockType, chunks_map::ChunksType};

/// Tool for storing near chunks
pub struct NearChunksData {
    pub forward: Option<ColumnDataLockType>,
    pub behind: Option<ColumnDataLockType>,
    pub left: Option<ColumnDataLockType>,
    pub right: Option<ColumnDataLockType>,
}

impl NearChunksData {
    pub fn new(chunks: &ChunksType, pos: &ChunkPosition) -> Self {
        Self {
            forward: NearChunksData::get_data(chunks, &ChunkPosition::new(pos.x - 1, pos.z)),
            behind: NearChunksData::get_data(chunks, &ChunkPosition::new(pos.x + 1, pos.z)),
            left: NearChunksData::get_data(chunks, &ChunkPosition::new(pos.x, pos.z - 1)),
            right: NearChunksData::get_data(chunks, &ChunkPosition::new(pos.x, pos.z + 1)),
        }
    }

    pub fn is_full(&self) -> bool {
        self.forward.is_some() && self.behind.is_some() && self.left.is_some() && self.right.is_some()
    }

    fn get_data(chunks: &ChunksType, pos: &ChunkPosition) -> Option<ColumnDataLockType> {
        match chunks.get(pos) {
            Some(c) => Some(c.read().get_data_lock().clone()),
            None => None,
        }
    }
}
