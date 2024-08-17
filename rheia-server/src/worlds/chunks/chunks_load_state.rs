use ahash::AHashMap;
use bevy::prelude::Entity;
use common::{chunks::chunk_position::ChunkPosition, utils::vec_remove_item};

/// Idia was taken from
/// https://github.com/feather-rs/feather
/// feather/common/src/chunk/loading.rs
#[derive(Default)]
pub struct ChunksLoadState {
    pub(crate) by_chunk: AHashMap<ChunkPosition, Vec<Entity>>,
    by_entity: AHashMap<Entity, Vec<ChunkPosition>>,
}

impl ChunksLoadState {
    /// Start tracking the chunk
    pub fn insert_ticket(&mut self, chunk: ChunkPosition, entity: Entity) {
        self.by_chunk.entry(chunk).or_default().push(entity);
        self.by_entity.entry(entity).or_default().push(chunk);
    }

    /// Stop tracking the chunk
    pub fn remove_ticket(&mut self, chunk: &ChunkPosition, entity: &Entity) {
        if let Some(vec) = self.by_chunk.get_mut(chunk) {
            vec_remove_item(vec, entity);
        }
        vec_remove_item(self.by_entity.get_mut(entity).unwrap(), &chunk);
    }

    /// Stop trachinkg all data by entity
    pub fn remove_all_entity_tickets(&mut self, entity: &Entity) {
        for (_pos, mut entities) in self.by_chunk.iter_mut() {
            vec_remove_item(&mut entities, entity);
        }
        self.by_entity.remove(&entity);
    }

    /// The number of players who are watching this chunk
    pub fn num_tickets(&self, chunk: &ChunkPosition) -> usize {
        match self.by_chunk.get(chunk) {
            Some(vec) => vec.len(),
            None => 0,
        }
    }

    /// Returns all entities that wathing the chunk
    pub fn get_chunk_watchers(&self, chunk: &ChunkPosition) -> Option<&Vec<Entity>> {
        match self.by_chunk.get(chunk) {
            Some(v) => Some(&v),
            None => None,
        }
    }

    /// Returns all chunks that player watching
    pub fn get_watching_chunks(&self, entity: &Entity) -> Option<&Vec<ChunkPosition>> {
        match self.by_entity.get(entity) {
            Some(v) => Some(&v),
            None => None,
        }
    }
}
