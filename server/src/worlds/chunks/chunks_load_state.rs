use ahash::AHashMap;
use common::utils::vec_remove_item;
use std::mem;

use super::chunks_map::ChunkPosition;

/// https://github.com/feather-rs/feather
/// feather/common/src/chunk/loading.rs
#[derive(Default)]
pub struct ChunksLoadState {
    pub(crate) by_chunk: AHashMap<ChunkPosition, Vec<u64>>,
    by_client: AHashMap<u64, Vec<ChunkPosition>>,
}

impl ChunksLoadState {
    pub fn insert_ticket(&mut self, chunk: ChunkPosition, client_id: u64) {
        self.by_chunk.entry(chunk).or_default().push(client_id);
        self.by_client.entry(client_id).or_default().push(chunk);
    }

    pub fn remove_ticket(&mut self, chunk: ChunkPosition, client_id: &u64) {
        if let Some(vec) = self.by_chunk.get_mut(&chunk) {
            vec_remove_item(vec, client_id);
        }
        vec_remove_item(self.by_client.get_mut(client_id).unwrap(), &chunk);
    }

    pub fn num_tickets(&self, chunk: ChunkPosition) -> usize {
        match self.by_chunk.get(&chunk) {
            Some(vec) => vec.len(),
            None => 0,
        }
    }

    pub fn take_entity_tickets(&mut self, client_id: u64) -> Vec<ChunkPosition> {
        self.by_client.get_mut(&client_id).map(mem::take).unwrap_or_default()
    }

    pub fn remove_chunk(&mut self, pos: ChunkPosition) {
        self.by_chunk.remove(&pos);
    }
}
