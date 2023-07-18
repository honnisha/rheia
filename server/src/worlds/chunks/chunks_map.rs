use ahash::{AHashMap};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use spiral::ManhattanIterator;
use std::{
    fmt::{self, Display, Formatter},
    sync::Arc,
    time::Duration,
};

use crate::{worlds::world_generator::WorldGenerator, CHUNKS_DESPAWN_TIMER, CHUNKS_DISTANCE};

use super::{chunks_load_state::ChunksLoadState, chunk_column::ChunkColumn};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize, Hash)]
pub struct ChunkPosition {
    pub x: i32,
    pub z: i32,
}

impl ChunkPosition {
    pub const fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }
}

impl Display for ChunkPosition {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "({}, {})", self.x, self.z)
    }
}

/// Container of 2d ChunkColumn's.
/// This container manages vision of the chunks
/// and responsible for load/unload chunks
#[derive(Default)]
pub struct ChunkMap {
    pub(crate) chunks: AHashMap<ChunkPosition, ChunkColumn>,
    pub chunks_load_state: ChunksLoadState,
}

impl ChunkMap {
    pub fn new() -> Self {
        Self::default()
    }

    /// Trigered when player is move between chunks or spawns/despawns
    /// for updating chunks vision
    /// to unload unused chunks
    pub fn update_chunks_render(&mut self, client_id: &u64, from: Option<&ChunkPosition>, to: Option<&ChunkPosition>) {
        if from.is_some() && to.is_some() && from.unwrap() == to.unwrap() {
            panic!("update_chunks_render from and to must be different chunks positions");
        }

        // Remove old chunks from player monitor
        if let Some(chunk_from) = from {
            let iter = ManhattanIterator::new(chunk_from.x, chunk_from.z, CHUNKS_DISTANCE);
            for (x, z) in iter {
                self.chunks_load_state
                    .remove_ticket(ChunkPosition::new(x, z), &client_id);
            }
        }

        // Add
        if let Some(chunk_to) = to {
            let iter = ManhattanIterator::new(chunk_to.x, chunk_to.z, CHUNKS_DISTANCE);
            for (x, z) in iter {
                let chunk_pos = ChunkPosition::new(x, z);
                self.chunks_load_state.insert_ticket(chunk_pos, client_id.clone());

                // Update despawn timer
                if let Some(c) = self.chunks.get_mut(&chunk_pos) {
                    c.despawn_timer = Duration::ZERO;
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
            if self.chunks_load_state.num_tickets(chunk_pos) == 0 {
                chunk_column.despawn_timer += delta;
            }
        }

        // Despawn chunks waiting for despawn
        self.chunks
            .retain(|&_chunk_pos, chunk_column| chunk_column.despawn_timer < CHUNKS_DESPAWN_TIMER);

        for &chunk_pos in self.chunks_load_state.by_chunk.keys() {
            if !self.chunks.contains_key(&chunk_pos) {
                let mut chunk_column = ChunkColumn::new(chunk_pos.clone(), world_slug.clone());
                chunk_column.load(world_generator.clone());
                self.chunks.insert(chunk_pos.clone(), chunk_column);
            }
        }
    }
}
