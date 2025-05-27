use crate::{
    CHUNK_SIZE, VERTICAL_SECTIONS,
    chunks::{
        block_position::ChunkBlockPosition,
        chunk_data::{BlockDataInfo, ChunkData, ChunkSectionData},
        chunk_position::ChunkPosition,
    },
};

use super::{default::WorldGeneratorSettings, traits::IWorldGenerator};

#[derive(Default)]
pub struct SphereWorldGenerator {}

impl IWorldGenerator for SphereWorldGenerator {
    type Error = String;

    fn create(_seed: Option<u64>, _settings: WorldGeneratorSettings) -> Result<Self, Self::Error> {
        Ok(Self {})
    }

    fn generate_chunk_data(&self, chunk_position: &ChunkPosition) -> ChunkData {
        let mut chunk_data: ChunkData = Default::default();
        for y in 0..VERTICAL_SECTIONS {
            let chunk_section = self.generate_section_data(&chunk_position, y);
            chunk_data.push_section(chunk_section);
        }
        chunk_data
    }
}

impl SphereWorldGenerator {
    fn generate_section_data(&self, _chunk_position: &ChunkPosition, _vertical_index: usize) -> ChunkSectionData {
        let mut section_data: ChunkSectionData = Default::default();

        let center = CHUNK_SIZE as f32 / 2.0;

        for x in 0_u8..(CHUNK_SIZE as u8) {
            for z in 0_u8..(CHUNK_SIZE as u8) {
                for y in 0_u8..(CHUNK_SIZE as u8) {
                    let dx = x as f32 - center;
                    let dy = y as f32 - center;
                    let dz = z as f32 - center;
                    let d = (dx * dx + dy * dy + dz * dz).sqrt();

                    if d < 10.0 {
                        let pos = ChunkBlockPosition::new(x, y, z);
                        section_data.insert(&pos, BlockDataInfo::create(1, None));
                    };
                }
            }
        }
        section_data
    }
}
