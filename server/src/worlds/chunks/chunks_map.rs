use ahash::AHashMap;
use common::VERTICAL_SECTIONS;
use flume::{Receiver, Sender};
use lazy_static::lazy_static;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use spiral::ManhattanIterator;
use std::{
    fmt::{self, Display, Formatter},
    time::Duration, sync::Arc,
};

use crate::{worlds::world_generator::WorldGenerator, CHUNKS_DESPAWN_TIMER, CHUNKS_DISTANCE};

use super::{chunk_section::ChunkSection, chunks_load_state::ChunksLoadState};

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
    chunks: AHashMap<ChunkPosition, ChunkColumn>,
    chunks_load_state: ChunksLoadState,
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

/// world_slug, chunk_position
pub type LoadedChunkType = (String, ChunkPosition);
lazy_static! {
    pub static ref LOADED_CHUNKS: (Sender<LoadedChunkType>, Receiver<LoadedChunkType>) = flume::unbounded();
}

pub struct ChunkColumn {
    chunk_position: ChunkPosition,
    world_slug: String,

    sections: [Option<ChunkSection>; VERTICAL_SECTIONS],
    despawn_timer: Duration,
}

impl ChunkColumn {
    fn new(chunk_position: ChunkPosition, world_slug: String) -> Self {
        Self {
            sections: Default::default(),
            despawn_timer: Duration::ZERO,
            chunk_position,
            world_slug,
        }
    }

    pub(crate) fn load(&mut self, world_generator: Arc<RwLock<WorldGenerator>>) {
        for y in 0..VERTICAL_SECTIONS {
            let mut chunk_section = ChunkSection::new();
            world_generator.read().generate_chunk_data(&mut chunk_section.chunk_data, &self.chunk_position, y);
            self.sections[y] = Some(chunk_section);
        }
        LOADED_CHUNKS
            .0
            .send((self.world_slug.clone(), self.chunk_position.clone()))
            .unwrap();
    }
}
