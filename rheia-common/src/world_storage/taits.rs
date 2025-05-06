use std::path::PathBuf;

use arrayvec::ArrayVec;

use crate::{chunks::chunk_position::ChunkPosition, world_generator::ChunkDataType, VERTICAL_SECTIONS};

pub(crate) type ChunkData = ArrayVec<Box<ChunkDataType>, VERTICAL_SECTIONS>;

#[derive(Default)]
pub struct WorldStorageSettings {
    server_data_path: PathBuf,
}

impl WorldStorageSettings {
    pub fn create(server_data_path: PathBuf) -> Self {
        Self { server_data_path }
    }
}

pub trait IWorldStorage {
    fn create(settings: WorldStorageSettings) -> Self;
    fn has_chunk_data(&self, chunk_position: &ChunkPosition) -> bool;
    fn load_chunk_data(&self, chunk_position: &ChunkPosition) -> ChunkData;
    fn save_chunk_data(&self, chunk_position: &ChunkPosition, data: &ChunkData);
}
