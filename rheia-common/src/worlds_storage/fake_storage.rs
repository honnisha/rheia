use crate::chunks::chunk_position::ChunkPosition;

use super::taits::{ChunkData, IWorldStorage, WorldInfo, WorldStorageSettings};

pub struct FakeWorldStorage {}

impl IWorldStorage for FakeWorldStorage {
    type Error = String;

    fn create(_world_slug: String, _seed: u64, _settings: &WorldStorageSettings) -> Result<Self, String> {
        Ok(Self {})
    }

    fn has_chunk_data(&self, _chunk_position: &ChunkPosition) -> bool {
        false
    }

    fn load_chunk_data(&self, _chunk_position: &ChunkPosition) -> ChunkData {
        unimplemented!()
    }

    fn save_chunk_data(&self, _chunk_position: &ChunkPosition, _data: &ChunkData) {}

    fn scan_worlds(_settings: &WorldStorageSettings) -> Result<Vec<WorldInfo>, String> {
        let worlds: Vec<WorldInfo> = Default::default();
        Ok(worlds)
    }
}
