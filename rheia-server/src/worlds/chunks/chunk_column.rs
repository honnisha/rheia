use arrayvec::ArrayVec;
use common::blocks::block_info::BlockInfo;
use common::chunks::block_position::ChunkBlockPosition;
use common::chunks::chunk_position::ChunkPosition;
use common::chunks::ChunkDataType;
use common::world_generator::default::WorldGenerator;
use common::worlds_storage::taits::{ChunkData, IWorldStorage};
use common::VERTICAL_SECTIONS;
use core::fmt;
use network::messages::ServerMessages;
use parking_lot::RwLock;
use std::fmt::Display;
use std::{sync::Arc, time::Duration};

use crate::network::runtime_plugin::RuntimePlugin;

use super::chunks_map::StorageLock;

pub struct ChunkColumn {
    chunk_position: ChunkPosition,
    world_slug: String,

    pub sections: ChunkData,
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

    pub fn get_chunk_position(&self) -> &ChunkPosition {
        &self.chunk_position
    }

    /// If chunk load his data
    pub(crate) fn is_loaded(&self) -> bool {
        self.loaded
    }

    pub fn change_block(&mut self, section: u32, chunk_block: ChunkBlockPosition, new_block_info: Option<BlockInfo>) {
        if section > VERTICAL_SECTIONS as u32 {
            panic!("Tried to change block in section {section} more than max {VERTICAL_SECTIONS}");
        }
        let s = self.sections.get_mut(section as usize).unwrap();
        match new_block_info {
            Some(i) => {
                s.insert(chunk_block, i);
            }
            None => {
                s.remove(&chunk_block);
            }
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

    pub(crate) fn build_network_format(&self) -> ServerMessages {
        let mut data: ArrayVec<Box<ChunkDataType>, VERTICAL_SECTIONS> = Default::default();
        for section in self.sections.iter() {
            data.push(section.clone());
        }
        return ServerMessages::ChunkSectionInfo {
            world_slug: self.world_slug.clone(),
            sections: data.into_inner().expect("data error"),
            chunk_position: self.chunk_position.clone(),
        };
    }
}

pub(crate) fn load_chunk(
    world_generator: Arc<RwLock<WorldGenerator>>,
    storage: StorageLock,
    chunk_column: Arc<RwLock<ChunkColumn>>,
    loaded_chunks: flume::Sender<ChunkPosition>,
) {
    rayon::spawn(move || {
        #[cfg(feature = "trace")]
        let _span = bevy_utils::tracing::info_span!("load_chunk").entered();

        if RuntimePlugin::is_stopped() {
            return;
        }

        let mut chunk_column = chunk_column.write();

        // Load from storage
        let index = match storage.lock().has_chunk_data(&chunk_column.chunk_position) {
            Ok(i) => i,
            Err(e) => {
                log::error!(target: "worlds", "&cChunk load error!");
                log::error!(target: "worlds", "Error: {}", e);
                RuntimePlugin::stop();
                return;
            },
        };
        if let Some(index) = index {
            chunk_column.sections = match storage.lock().load_chunk_data(index) {
                Ok(c) => c,
                Err(e) => {
                    log::error!(target: "worlds", "&cChunk load error!");
                    log::error!(target: "worlds", "Error: {}", e);
                    RuntimePlugin::stop();
                    return;
                }
            };
        }
        // Or generate new
        else {
            for y in 0..VERTICAL_SECTIONS {
                let chunk_section = world_generator
                    .read()
                    .generate_chunk_data(&chunk_column.chunk_position, y);
                chunk_column.sections.push(Box::new(chunk_section));
            }
        }
        chunk_column.loaded = true;

        if !cfg!(test) {
            loaded_chunks
                .send(chunk_column.chunk_position.clone())
                .expect("channel poisoned");
        }
    })
}
