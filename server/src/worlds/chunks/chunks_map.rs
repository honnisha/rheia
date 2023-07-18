use ahash::AHashMap;
use log::trace;
use parking_lot::RwLock;
use spiral::ManhattanIterator;
use std::{
    sync::Arc,
    time::Duration,
};

use crate::{worlds::world_generator::WorldGenerator, CHUNKS_DESPAWN_TIMER};

use super::{chunk_column::ChunkColumn, chunks_load_state::ChunksLoadState, chunk_position::ChunkPosition};

/// Container of 2d ChunkColumn's.
/// This container manages vision of the chunks
/// and responsible for load/unload chunks
#[derive(Default)]
pub struct ChunkMap {
    chunks: AHashMap<ChunkPosition, ChunkColumn>,
    chunks_load_state: ChunksLoadState,
}

impl ChunkMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_chunk_column(&self, chunk_position: &ChunkPosition) -> Option<&ChunkColumn> {
        match self.chunks.get(chunk_position) {
            Some(c) => Some(&c),
            None => None,
        }
    }

    pub fn take_chunks_clients(&self, chunk_position: &ChunkPosition) -> Option<&Vec<u64>> {
        self.chunks_load_state.take_chunks_clients(&chunk_position)
    }

    /// Trigered when player is move between chunks or spawns/despawns
    /// for updating chunks vision
    /// to unload unused chunks
    pub fn update_chunks_render(
        &mut self,
        client_id: &u64,
        from: Option<&ChunkPosition>,
        to: Option<&ChunkPosition>,
        chunks_distance: u16,
    ) {
        if from.is_some() && to.is_some() && from.unwrap() == to.unwrap() {
            panic!("update_chunks_render from and to must be different chunks positions");
        }

        // Remove old chunks from player monitor
        if let Some(chunk_from) = from {
            let iter = ManhattanIterator::new(chunk_from.x, chunk_from.z, chunks_distance);
            for (x, z) in iter {
                self.chunks_load_state
                    .remove_ticket(ChunkPosition::new(x, z), &client_id);
            }
        }

        // Add new tickets
        if let Some(chunk_to) = to {
            let iter = ManhattanIterator::new(chunk_to.x, chunk_to.z, chunks_distance);
            for (x, z) in iter {
                let chunk_pos = ChunkPosition::new(x, z);
                self.chunks_load_state.insert_ticket(chunk_pos, client_id.clone());

                // Update despawn timer
                if let Some(c) = self.chunks.get_mut(&chunk_pos) {
                    c.set_despawn_timer(Duration::ZERO);
                }
            }
        }
    }

    pub fn update_chunks(
        &mut self,
        delta: Duration,
        world_slug: &String,
        world_generator: Arc<RwLock<WorldGenerator>>,
    ) {
        // Update chunks despawn timer
        for (&chunk_pos, chunk_column) in self.chunks.iter_mut() {
            if self.chunks_load_state.num_tickets(&chunk_pos) == 0 {
                chunk_column.increase_despawn_timer(delta);
            }
        }

        // Despawn chunks waiting for despawn
        self.chunks.retain(|&chunk_pos, chunk_column| {
            let keep = chunk_column.get_despawn_timer() < &CHUNKS_DESPAWN_TIMER;
            if !keep {
                trace!(target: "chunks", "Chunk {} despawned", chunk_pos);
            }
            keep
        });

        // Send to load new chunks
        for (chunk_pos, players) in self.chunks_load_state.by_chunk.iter() {
            if players.len() == 0{
                continue;
            }

            if !self.chunks.contains_key(&chunk_pos) {
                let mut chunk_column = ChunkColumn::new(chunk_pos.clone(), world_slug.clone());
                trace!(target: "chunks", "Send chunk {} to load", chunk_pos);
                chunk_column.load(world_generator.clone());
                self.chunks.insert(chunk_pos.clone(), chunk_column);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use parking_lot::RwLock;
    use std::time::Duration;

    use crate::{worlds::world_generator::WorldGenerator, CHUNKS_DESPAWN_TIMER};

    use super::{ChunkMap, ChunkPosition};

    #[test]
    fn test_tickets_spawn_despawn() {
        let mut chunk_map = ChunkMap::default();
        let client_id = 0_u64;
        let pos = ChunkPosition::new(0, 0);
        let chunks_distance = 3_u16;

        // Spawn
        chunk_map.update_chunks_render(&client_id, None, Some(&pos), chunks_distance);
        assert_eq!(chunk_map.chunks_load_state.num_tickets(&pos), 1);
        assert_eq!(chunk_map.chunks_load_state.take_entity_tickets(&client_id).len(), 13);

        // despawn
        chunk_map.update_chunks_render(&client_id, Some(&pos), None, chunks_distance);
        assert_eq!(chunk_map.chunks_load_state.num_tickets(&pos), 0);
        assert_eq!(chunk_map.chunks_load_state.take_entity_tickets(&client_id).len(), 0);
    }

    #[test]
    fn test_update_chunks() {
        let mut chunk_map = ChunkMap::default();
        let world_generator = Arc::new(RwLock::new(WorldGenerator::default()));
        let world_slug = "default".to_string();
        let client_id = 0_u64;
        let pos = ChunkPosition::new(0, 0);

        chunk_map
            .chunks_load_state
            .insert_ticket(pos.clone(), client_id.clone());
        chunk_map.update_chunks(Duration::from_secs(1), &world_slug, world_generator.clone());
        assert_eq!(chunk_map.chunks.len(), 1, "One chunk must be created");

        chunk_map.chunks.get_mut(&pos).unwrap().set_despawn_timer(CHUNKS_DESPAWN_TIMER);

        chunk_map.chunks_load_state.remove_ticket(pos.clone(), &client_id);
        chunk_map.update_chunks(Duration::from_secs(1), &world_slug, world_generator.clone());
        assert_eq!(
            chunk_map.chunks.len(),
            0,
            "Because despawn_timer is fill - chunk must be unloaded"
        );
    }
}
