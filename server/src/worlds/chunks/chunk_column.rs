use crate::worlds::world_generator::WorldGenerator;
use arrayvec::ArrayVec;
use common::chunks::chunk_position::ChunkPosition;
use common::{network::ChunkDataType, VERTICAL_SECTIONS};
use core::fmt;
use flume::{Receiver, Sender};
use lazy_static::lazy_static;
use parking_lot::RwLock;
use std::fmt::Display;
use std::{collections::HashMap, sync::Arc, time::Duration};

/// world_slug, chunk_position
pub type LoadedChunkType = (String, ChunkPosition);
lazy_static! {
    pub static ref LOADED_CHUNKS: (Sender<LoadedChunkType>, Receiver<LoadedChunkType>) = flume::unbounded();
}

pub struct ChunkColumn {
    chunk_position: ChunkPosition,
    world_slug: String,

    sections: ArrayVec<ChunkDataType, VERTICAL_SECTIONS>,
    despawn_timer: Arc<RwLock<Duration>>,
    loaded: bool,
}

impl Display for ChunkColumn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ChunkColumn{{x:{} z:{} despawn_timer:{}}}",
            self.chunk_position.x,
            self.chunk_position.z,
            self.despawn_timer.read().as_secs_f32()
        )
    }
}

impl ChunkColumn {
    pub(crate) fn new(chunk_position: ChunkPosition, world_slug: String) -> Self {
        Self {
            sections: Default::default(),
            despawn_timer: Arc::new(RwLock::new(Duration::ZERO)),
            chunk_position,
            world_slug,
            loaded: false,
        }
    }

    pub(crate) fn is_for_despawn(&self, duration: Duration) -> bool {
        *self.despawn_timer.read() >= duration
    }

    pub(crate) fn set_despawn_timer(&self, new_despawn: Duration) {
        *self.despawn_timer.write() = new_despawn;
    }

    pub(crate) fn increase_despawn_timer(&self, new_despawn: Duration) {
        *self.despawn_timer.write() += new_despawn;
    }

    pub(crate) fn build_network_format(&self) -> [ChunkDataType; VERTICAL_SECTIONS] {
        self.sections.clone().into_inner().unwrap()
    }
}

pub(crate) fn load_chunk(world_generator: Arc<RwLock<WorldGenerator>>, chunk_column: Arc<RwLock<ChunkColumn>>) {
    rayon::spawn(move || {
        let mut chunk_column = chunk_column.write();

        for y in 0..VERTICAL_SECTIONS {
            let mut chunk_section: ChunkDataType = HashMap::new();
            world_generator
                .read()
                .generate_chunk_data(&mut chunk_section, &chunk_column.chunk_position, y);
            chunk_column.sections.push(chunk_section);
        }
        chunk_column.loaded = true;
        LOADED_CHUNKS
            .0
            .send((chunk_column.world_slug.clone(), chunk_column.chunk_position.clone()))
            .unwrap();
    })
}
