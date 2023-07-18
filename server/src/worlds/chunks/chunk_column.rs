use crate::worlds::world_generator::WorldGenerator;
use arrayvec::ArrayVec;
use common::{network::ChunkDataType, VERTICAL_SECTIONS};
use core::fmt;
use flume::{Receiver, Sender};
use lazy_static::lazy_static;
use parking_lot::RwLock;
use std::fmt::Display;
use std::{collections::HashMap, sync::Arc, time::Duration};

use super::chunk_position::ChunkPosition;

/// world_slug, chunk_position
pub type LoadedChunkType = (String, ChunkPosition);
lazy_static! {
    pub static ref LOADED_CHUNKS: (Sender<LoadedChunkType>, Receiver<LoadedChunkType>) = flume::unbounded();
}

pub struct ChunkColumn {
    chunk_position: ChunkPosition,
    world_slug: String,

    sections: ArrayVec<ChunkDataType, VERTICAL_SECTIONS>,
    despawn_timer: Duration,
    loaded: bool,
}

impl Display for ChunkColumn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ChunkColumn{{x:{} z:{} despawn_timer:{}}}",
            self.chunk_position.x,
            self.chunk_position.z,
            self.despawn_timer.as_secs_f32()
        )
    }
}

impl ChunkColumn {
    pub(crate) fn new(chunk_position: ChunkPosition, world_slug: String) -> Self {
        Self {
            sections: Default::default(),
            despawn_timer: Duration::ZERO,
            chunk_position,
            world_slug,
            loaded: false,
        }
    }

    pub(crate) fn get_despawn_timer(&self) -> &Duration {
        &self.despawn_timer
    }

    pub(crate) fn set_despawn_timer(&mut self, new_despawn: Duration) {
        self.despawn_timer = new_despawn;
    }

    pub(crate) fn increase_despawn_timer(&mut self, new_despawn: Duration) {
        self.despawn_timer += new_despawn;
    }

    pub(crate) fn load(&mut self, world_generator: Arc<RwLock<WorldGenerator>>) {
        //load_chunk(world_generator, self.loaded.clone());

        for y in 0..VERTICAL_SECTIONS {
            let mut chunk_section: ChunkDataType = HashMap::new();
            world_generator
                .read()
                .generate_chunk_data(&mut chunk_section, &self.chunk_position, y);
            self.sections.push(chunk_section);
        }
        self.loaded = true;
        LOADED_CHUNKS
            .0
            .send((self.world_slug.clone(), self.chunk_position.clone()))
            .unwrap();
    }

    pub(crate) fn build_network_format(&self) -> [ChunkDataType; VERTICAL_SECTIONS] {
        self.sections.clone().into_inner().unwrap()
    }
}

//fn load_chunk(world_generator: Arc<RwLock<WorldGenerator>>, loaded: Arc<AtomicBool>, chunk_position: ChunkPosition) {
//}
