use common::chunks::chunk_position::ChunkPosition;

use super::{chunk::ColumnDataType, godot_chunks_container::ChunksType};

/// Tool for storing near chunks
pub struct NearChunksData {
    pub forward: Option<ColumnDataType>,
    pub behind: Option<ColumnDataType>,
    pub left: Option<ColumnDataType>,
    pub right: Option<ColumnDataType>,
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

    fn get_data(chunks: &ChunksType, pos: &ChunkPosition) -> Option<ColumnDataType> {
        match chunks.get(pos) {
            Some(c) => Some(c.borrow().get_chunk_data().clone()),
            None => None,
        }
    }
}
