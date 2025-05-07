use std::path::PathBuf;

use arrayvec::ArrayVec;

use crate::{chunks::chunk_position::ChunkPosition, world_generator::ChunkDataType, VERTICAL_SECTIONS};

pub(crate) type ChunkData = ArrayVec<Box<ChunkDataType>, VERTICAL_SECTIONS>;

#[derive(Default)]
pub struct WorldStorageSettings {
    data_path: PathBuf,
}

impl WorldStorageSettings {
    pub fn create(data_path: PathBuf) -> Self {
        Self { data_path }
    }

    pub fn get_data_path(&self) -> &PathBuf {
        &self.data_path
    }
}

pub struct WorldInfo {
    pub slug: String,
    pub seed: u64,
}

pub trait IWorldStorage: Sized {
    type Error;

    fn create(world_slug: String, seed: u64, settings: &WorldStorageSettings) -> Result<Self, Self::Error>;
    fn has_chunk_data(&self, chunk_position: &ChunkPosition) -> bool;
    fn load_chunk_data(&self, chunk_position: &ChunkPosition) -> ChunkData;
    fn save_chunk_data(&self, chunk_position: &ChunkPosition, data: &ChunkData);

    fn scan_worlds(settings: &WorldStorageSettings) -> Result<Vec<WorldInfo>, String>;
}
