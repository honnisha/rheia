use std::collections::BTreeMap;

use crate::chunks::{
    chunk_data::{BlockIndexType, ChunkData},
    chunk_position::ChunkPosition,
};

use super::taits::{IWorldStorage, WorldInfo, WorldStorageSettings};

pub struct FakeWorldStorage {}

impl IWorldStorage for FakeWorldStorage {
    type Error = String;
    type PrimaryKey = ();

    fn create(_world_slug: String, _seed: u64, _settings: &WorldStorageSettings) -> Result<Self, String> {
        Ok(Self {})
    }

    fn has_chunk_data(&self, _chunk_position: &ChunkPosition) -> Result<Option<Self::PrimaryKey>, String> {
        Ok(None)
    }

    fn load_chunk_data(&self, _chunk_id: Self::PrimaryKey) -> Result<ChunkData, String> {
        unimplemented!()
    }

    fn save_chunk_data(&self, _chunk_position: &ChunkPosition, _data: &ChunkData) -> Result<Self::PrimaryKey, String> {
        Ok(())
    }

    fn scan_worlds(_settings: &WorldStorageSettings) -> Result<Vec<WorldInfo>, String> {
        let worlds: Vec<WorldInfo> = Default::default();
        Ok(worlds)
    }

    fn delete(&self, _settings: &WorldStorageSettings) -> Result<(), String> {
        Ok(())
    }

    fn validate_block_id_map(
        _world_slug: String,
        _settings: &WorldStorageSettings,
        _block_id_map: &BTreeMap<BlockIndexType, String>,
    ) -> Result<(), String> {
        todo!()
    }
}
