use ahash::AHashMap;
use bevy::prelude::Entity;
use common::{utils::vec_remove_item, chunks::chunk_position::ChunkPosition};
use std::mem;

/// Idia was taken from
/// https://github.com/feather-rs/feather
/// feather/common/src/chunk/loading.rs
#[derive(Default)]
pub struct ChunksLoadState {
    pub(crate) by_chunk: AHashMap<ChunkPosition, Vec<Entity>>,
    by_entity: AHashMap<Entity, Vec<ChunkPosition>>,
}

impl ChunksLoadState {
    pub fn insert_ticket(&mut self, chunk: ChunkPosition, entity: Entity) {
        self.by_chunk.entry(chunk).or_default().push(entity);
        self.by_entity.entry(entity).or_default().push(chunk);
    }

    pub fn remove_ticket(&mut self, chunk: ChunkPosition, entity: &Entity) {
        if let Some(vec) = self.by_chunk.get_mut(&chunk) {
            vec_remove_item(vec, entity);
        }
        vec_remove_item(self.by_entity.get_mut(entity).unwrap(), &chunk);
    }

    pub fn num_tickets(&self, chunk: &ChunkPosition) -> usize {
        match self.by_chunk.get(chunk) {
            Some(vec) => vec.len(),
            None => 0,
        }
    }

    pub fn take_chunks_entities(&self, chunk: &ChunkPosition) -> Option<&Vec<Entity>> {
        match self.by_chunk.get(chunk) {
            Some(v) => Some(&v),
            None => None,
        }
    }

    pub fn take_entity_chunks(&self, entity: &Entity) -> Option<&Vec<ChunkPosition>> {
        match self.by_entity.get(entity) {
            Some(v) => Some(&v),
            None => None,
        }
    }

    pub fn _take_chunks_entitys_mut(&mut self, chunk: &ChunkPosition) -> Vec<Entity> {
        self.by_chunk.get_mut(chunk).map(mem::take).unwrap_or_default()
    }

    #[allow(dead_code)]
    pub fn take_entity_tickets(&mut self, entity: &Entity) -> Vec<ChunkPosition> {
        self.by_entity.get_mut(entity).map(mem::take).unwrap_or_default()
    }

    pub fn _remove_chunk(&mut self, pos: ChunkPosition) {
        self.by_chunk.remove(&pos);
    }
}
