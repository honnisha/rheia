use std::{collections::hash_map::Iter, sync::Arc};

use ahash::AHashMap;
use common::chunks::chunk_position::ChunkPosition;
use parking_lot::RwLock;

use super::chunk_column::ChunkColumn;

pub type ChunkLock = Arc<RwLock<ChunkColumn>>;
type ChunksType = AHashMap<ChunkPosition, ChunkLock>;

#[derive(Default)]
pub struct ChunkMap {
    chunks: ChunksType,
}

impl ChunkMap {
    pub fn get_chunk(&self, chunk_position: &ChunkPosition) -> Option<&ChunkLock> {
        match self.chunks.get(chunk_position) {
            Some(c) => Some(c),
            None => None,
        }
    }

    pub fn insert(&mut self, chunk_position: ChunkPosition, column: ChunkColumn) {
        self.chunks.insert(chunk_position, Arc::new(RwLock::new(column)));
    }

    pub fn iter(&self) -> Iter<ChunkPosition, ChunkLock> {
        self.chunks.iter()
    }
}
