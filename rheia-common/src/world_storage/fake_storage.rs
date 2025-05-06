use crate::chunks::chunk_position::ChunkPosition;

use super::taits::{ChunkData, IWorldStorage, WorldStorageSettings};

#[derive(Default)]
pub struct FakeWorldStorage {}

impl IWorldStorage for FakeWorldStorage {
    fn create(_settings: WorldStorageSettings) -> Self {
        Self {}
    }

    fn has_chunk_data(&self, _chunk_position: &ChunkPosition) -> bool {
        false
    }

    fn load_chunk_data(&self, _chunk_position: &ChunkPosition) -> ChunkData {
        unimplemented!()
    }

    fn save_chunk_data(&self, _chunk_position: &ChunkPosition, _data: &ChunkData) {}
}
