use std::{sync::Arc, time::Duration, collections::HashMap};

use common::{network::ChunkDataType, VERTICAL_SECTIONS};
use flume::{Receiver, Sender};
use lazy_static::lazy_static;
use parking_lot::RwLock;

use crate::worlds::world_generator::WorldGenerator;

use super::chunks_map::ChunkPosition;

/// world_slug, chunk_position
pub type LoadedChunkType = (String, ChunkPosition);
lazy_static! {
    pub static ref LOADED_CHUNKS: (Sender<LoadedChunkType>, Receiver<LoadedChunkType>) = flume::unbounded();
}

pub struct ChunkColumn {
    chunk_position: ChunkPosition,
    world_slug: String,

    pub(crate) sections: [Option<ChunkDataType>; VERTICAL_SECTIONS],
    pub(crate) despawn_timer: Duration,
}

impl ChunkColumn {
    pub(crate) fn new(chunk_position: ChunkPosition, world_slug: String) -> Self {
        Self {
            sections: Default::default(),
            despawn_timer: Duration::ZERO,
            chunk_position,
            world_slug,
        }
    }

    pub(crate) fn load(&mut self, world_generator: Arc<RwLock<WorldGenerator>>) {
        for y in 0..VERTICAL_SECTIONS {
            let mut chunk_section: ChunkDataType = HashMap::new();
            world_generator
                .read()
                .generate_chunk_data(&mut chunk_section, &self.chunk_position, y);
            self.sections[y] = Some(chunk_section);
        }
        LOADED_CHUNKS
            .0
            .send((self.world_slug.clone(), self.chunk_position.clone()))
            .unwrap();
    }
}
