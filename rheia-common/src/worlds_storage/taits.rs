use crate::chunks::{
    chunk_data::{BlockIndexType, ChunkData},
    chunk_position::ChunkPosition,
};
use std::{collections::BTreeMap, path::PathBuf};

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
    type PrimaryKey;

    fn create(world_slug: String, seed: u64, settings: &WorldStorageSettings) -> Result<Self, Self::Error>;
    fn has_chunk_data(&self, chunk_position: &ChunkPosition) -> Result<Option<Self::PrimaryKey>, String>;
    fn load_chunk_data(&self, chunk_id: Self::PrimaryKey) -> Result<ChunkData, String>;
    fn save_chunk_data(&self, chunk_position: &ChunkPosition, data: &ChunkData) -> Result<Self::PrimaryKey, String>;
    fn delete(&self, settings: &WorldStorageSettings) -> Result<(), String>;

    fn scan_worlds(settings: &WorldStorageSettings) -> Result<Vec<WorldInfo>, String>;

    fn validate_block_id_map(
        world_slug: String,
        settings: &WorldStorageSettings,
        block_id_map: &BTreeMap<BlockIndexType, String>,
    ) -> Result<(), String>;
}
