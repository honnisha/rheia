use ahash::AHashMap;
use bevy::prelude::Entity;
use common::{chunks::chunk_position::ChunkPosition, utils::vec_remove_item};
use log::trace;
use parking_lot::{RwLock, RwLockReadGuard};
use spiral::ManhattanIterator;
use std::{sync::Arc, time::Duration};

use crate::{
    worlds::{chunks::chunk_column::load_chunk, world_generator::WorldGenerator},
    CHUNKS_DESPAWN_TIMER,
};

use super::{chunk_column::ChunkColumn, chunks_load_state::ChunksLoadState};

pub type MapChunksType = AHashMap<ChunkPosition, Arc<RwLock<ChunkColumn>>>;

/// Container of 2d ChunkColumn's.
/// This container manages vision of the chunks
/// and responsible for load/unload chunks
#[derive(Default)]
pub struct ChunkMap {
    chunks: MapChunksType,
    chunks_load_state: ChunksLoadState,
}

pub type ChunkSectionType<'a> = RwLockReadGuard<'a, ChunkColumn>;

impl ChunkMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn count(&self) -> usize {
        self.chunks.len()
    }

    pub fn get_chunks(&self) -> &MapChunksType {
        &self.chunks
    }

    pub fn get_chunk_column(&self, chunk_position: &ChunkPosition) -> Option<ChunkSectionType> {
        match self.chunks.get(chunk_position) {
            Some(c) => Some(c.read()),
            None => None,
        }
    }

    pub fn take_chunks_entities(&self, chunk_position: &ChunkPosition) -> Option<&Vec<Entity>> {
        self.chunks_load_state.take_chunks_entities(&chunk_position)
    }

    /// Create player in the world
    pub fn start_chunks_render(&mut self, entity: Entity, to: &ChunkPosition, chunks_distance: u16) {
        let iter = ManhattanIterator::new(to.x as i32, to.z as i32, chunks_distance as i32);
        for (x, z) in iter {
            let chunk_pos = ChunkPosition::new(x as i64, z as i64);
            self.chunks_load_state.insert_ticket(chunk_pos, entity.clone());

            // Update despawn timer
            if let Some(chunk_column) = self.chunks.get_mut(&chunk_pos) {
                chunk_column.read().set_despawn_timer(Duration::ZERO);
            }
        }
    }

    /// Trigered when player is move between chunks
    /// for updating chunks vision
    /// to unload unused chunks
    ///
    /// Returns unwatchd chunks
    pub fn update_chunks_render(
        &mut self,
        entity: Entity,
        from: &ChunkPosition,
        to: &ChunkPosition,
        chunks_distance: u16,
    ) -> Vec<ChunkPosition> {
        if from == to {
            panic!("update_chunks_render from and to must be different chunks positions");
        }

        let mut old_chunks = self.chunks_load_state.take_entity_chunks(&entity).unwrap().clone();

        // Add new tickets
        let iter = ManhattanIterator::new(to.x as i32, to.z as i32, chunks_distance as i32);
        for (x, z) in iter {
            let chunk_pos = ChunkPosition::new(x as i64, z as i64);

            // If its new chunk
            if !old_chunks.contains(&chunk_pos) {
                // Start keeping this chunk
                self.chunks_load_state.insert_ticket(chunk_pos, entity.clone());

                // Update despawn timer
                if let Some(chunk_column) = self.chunks.get_mut(&chunk_pos) {
                    chunk_column.read().set_despawn_timer(Duration::ZERO);
                }
            } else {
                // Remove chunk outside of side of view
                vec_remove_item(&mut old_chunks, &chunk_pos);
            }
        }

        for chunk in old_chunks.iter() {
            // Stop keeping this chunk
            self.chunks_load_state.remove_ticket(&chunk, &entity);
        }

        return old_chunks;
    }

    /// Player stop watch the world (despawn or move to another world)
    pub fn stop_chunks_render(&mut self, entity: Entity) {
        self.chunks_load_state.remove_all_entity_tickets(&entity);
    }

    /// Update chunks: load or despawn
    pub fn update_chunks(
        &mut self,
        delta: Duration,
        world_slug: &String,
        world_generator: Arc<RwLock<WorldGenerator>>,
    ) {
        // Update chunks despawn timer
        // Increase ONLY of noone looking at the chunk
        for (&chunk_pos, chunk_column) in self.chunks.iter_mut() {
            if self.chunks_load_state.num_tickets(&chunk_pos) == 0 {
                chunk_column.read().increase_despawn_timer(delta);
            }
        }

        // Despawn chunks waiting for despawn
        self.chunks.retain(|&chunk_pos, chunk_column| {
            let for_despawn = chunk_column.read().is_for_despawn(CHUNKS_DESPAWN_TIMER);
            if for_despawn {
                trace!(target: "chunks", "Chunk {} despawned", chunk_pos);
            }
            !for_despawn
        });

        // Send to load new chunks
        for (chunk_pos, players) in self.chunks_load_state.by_chunk.iter() {
            if players.len() == 0 {
                continue;
            }

            if !self.chunks.contains_key(&chunk_pos) {
                let chunk_column = Arc::new(RwLock::new(ChunkColumn::new(chunk_pos.clone(), world_slug.clone())));

                trace!(target: "chunks", "Send chunk {} to load", chunk_pos);
                load_chunk(world_generator.clone(), chunk_column.clone());
                self.chunks.insert(chunk_pos.clone(), chunk_column);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use bevy::prelude::Entity;
    use parking_lot::RwLock;
    use std::time::Duration;

    use crate::{worlds::world_generator::WorldGenerator, CHUNKS_DESPAWN_TIMER};

    use super::{ChunkMap, ChunkPosition};

    #[test]
    fn test_tickets_spawn_despawn() {
        let mut chunk_map = ChunkMap::default();
        let entity = Entity::from_raw(0);
        let chunks_distance = 2_u16;

        // Spawn
        let pos = ChunkPosition::new(0, 0);
        chunk_map.start_chunks_render(entity, &pos, chunks_distance);
        let chunks = chunk_map.chunks_load_state.take_entity_chunks(&entity).unwrap();
        assert_eq!(chunks.len(), 5);
        assert_eq!(chunks.contains(&ChunkPosition::new(0, 0)), true);
        assert_eq!(chunks.contains(&ChunkPosition::new(0, 1)), true);
        assert_eq!(chunks.contains(&ChunkPosition::new(0, -1)), true);
        assert_eq!(chunks.contains(&ChunkPosition::new(1, 0)), true);
        assert_eq!(chunks.contains(&ChunkPosition::new(-1, 0)), true);
        assert_eq!(chunk_map.chunks_load_state.num_tickets(&pos), 1);

        // Move
        let new_pos = ChunkPosition::new(1, 0);
        let abandoned_chunks = chunk_map.update_chunks_render(entity, &pos, &new_pos, chunks_distance);
        let chunks = chunk_map.chunks_load_state.take_entity_chunks(&entity).unwrap();
        assert_eq!(chunks.len(), 5);
        assert_eq!(chunks.contains(&ChunkPosition::new(1, 0)), true);
        assert_eq!(chunks.contains(&ChunkPosition::new(1, 1)), true);
        assert_eq!(chunks.contains(&ChunkPosition::new(1, -1)), true);
        assert_eq!(chunks.contains(&ChunkPosition::new(2, 0)), true);
        assert_eq!(chunks.contains(&ChunkPosition::new(0, 0)), true);
        assert_eq!(chunk_map.chunks_load_state.num_tickets(&new_pos), 1);

        assert_eq!(abandoned_chunks.len(), 3);
        assert_eq!(abandoned_chunks.contains(&ChunkPosition::new(-1, 0)), true);
        assert_eq!(abandoned_chunks.contains(&ChunkPosition::new(0, 1)), true);
        assert_eq!(abandoned_chunks.contains(&ChunkPosition::new(0, -1)), true);

        // despawn
        chunk_map.stop_chunks_render(entity);
        assert_eq!(chunk_map.chunks_load_state.take_entity_chunks(&entity).is_none(), true);
        assert_eq!(chunk_map.chunks_load_state.num_tickets(&new_pos), 0);
    }

    #[test]
    fn test_update_chunks() {
        let mut chunk_map = ChunkMap::default();
        let world_generator = Arc::new(RwLock::new(WorldGenerator::default()));
        let world_slug = "default".to_string();
        let entity = Entity::from_raw(0);
        let pos = ChunkPosition::new(0, 0);

        chunk_map.chunks_load_state.insert_ticket(pos.clone(), entity.clone());
        chunk_map.update_chunks(Duration::from_secs(1), &world_slug, world_generator.clone());
        assert_eq!(chunk_map.chunks.len(), 1, "One chunk must be created");

        chunk_map
            .get_chunk_column(&pos)
            .unwrap()
            .set_despawn_timer(CHUNKS_DESPAWN_TIMER);

        chunk_map.chunks_load_state.remove_ticket(&pos, &entity);
        chunk_map.update_chunks(Duration::from_secs(1), &world_slug, world_generator.clone());
        assert_eq!(
            chunk_map.chunks.len(),
            0,
            "Because despawn_timer is fill - chunk must be unloaded"
        );
    }
}
