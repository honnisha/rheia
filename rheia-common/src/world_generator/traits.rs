use crate::chunks::{chunk_data::ChunkData, chunk_position::ChunkPosition};

use super::default::WorldGeneratorSettings;

pub trait IWorldGenerator: Sized {
    type Error;

    fn create(seed: Option<u64>, settings: WorldGeneratorSettings) -> Result<Self, Self::Error>;
    fn generate_chunk_data(&self, chunk_position: &ChunkPosition) -> ChunkData;
}
