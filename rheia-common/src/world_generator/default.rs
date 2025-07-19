use crate::{
    chunks::{
        block_position::ChunkBlockPosition,
        chunk_data::{BlockDataInfo, ChunkData, ChunkSectionData},
        chunk_position::ChunkPosition,
    },
    default_blocks_ids::BlockID,
    CHUNK_SIZE, VERTICAL_SECTIONS,
};
use bracket_lib::random::RandomNumberGenerator;
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

use super::{noise::{GeneratedNoise, Noise}, traits::IWorldGenerator};

#[serde_inline_default]
#[derive(Default, Serialize, Deserialize)]
pub struct WorldGeneratorSettings {
    #[serde_inline_default(60.0)]
    ground_level: f32,
    #[serde_inline_default(57.0)]
    water_level: f32,

    surface_noise: Noise,
    #[serde_inline_default(10.0)]
    surface_multiplier: f32,

    river_noise: Noise,
    #[serde_inline_default(10.0)]
    river_multiplier: f32,

    stream_noise: Noise,
    stream_second_noise: Noise,
    #[serde_inline_default(10.0)]
    stream_multiplier: f32,

    #[serde_inline_default(5.0)]
    sand_threshold: f32,
}

pub struct WorldGenerator {
    surface_noise: GeneratedNoise,
    river_noise: GeneratedNoise,
    stream_noise: GeneratedNoise,
    stream_second_noise: GeneratedNoise,
    settings: WorldGeneratorSettings,
}

impl IWorldGenerator for WorldGenerator {
    type Error = String;

    fn create(seed: Option<u64>, settings: WorldGeneratorSettings) -> Result<Self, Self::Error> {
        let seed = match seed {
            Some(s) => s,
            None => {
                let mut rng = RandomNumberGenerator::new();
                rng.next_u64()
            }
        };

        Ok(Self {
            surface_noise: settings.surface_noise.generate(seed),
            river_noise: settings.river_noise.generate(seed),
            stream_noise: settings.stream_noise.generate(seed),
            stream_second_noise: settings.stream_second_noise.generate(seed),
            settings: settings,
        })
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

impl WorldGenerator {
    fn generate_section_data(&self, chunk_position: &ChunkPosition, vertical_index: usize) -> ChunkSectionData {
        let mut section_data: ChunkSectionData = Default::default();

        for x in 0_u8..(CHUNK_SIZE as u8) {
            for z in 0_u8..(CHUNK_SIZE as u8) {
                let x_map = x as f32 + (chunk_position.x as f32 * CHUNK_SIZE as f32);
                let z_map = z as f32 + (chunk_position.z as f32 * CHUNK_SIZE as f32);
                let surface = self.surface_noise.get_noise(x_map, z_map) * self.settings.surface_multiplier
                    + self.settings.ground_level;

                // Множитель для рек, превращающий их в реки
                let river_noise = self.river_noise.get_noise(x_map, z_map);

                // Реки
                let stream_noise = self.stream_noise.get_noise(x_map, z_map);
                let stream_second_noise = self.stream_second_noise.get_noise(x_map, z_map);
                let stream = (stream_noise + (stream_noise * stream_second_noise) * (1.0 + river_noise))
                    * self.settings.stream_multiplier;

                for y in 0_u8..(CHUNK_SIZE as u8) {
                    let pos = ChunkBlockPosition::new(x, y, z);

                    let y_global = y as f32 + (vertical_index as f32 * CHUNK_SIZE as f32);

                    if y_global < surface - stream {
                        section_data.insert(&pos, BlockDataInfo::create(BlockID::Grass.id(), None));

                        if stream > self.settings.sand_threshold {
                            section_data.insert(&pos, BlockDataInfo::create(BlockID::Sand.id(), None));
                        }
                    } else if y_global < surface && y_global < self.settings.water_level {
                        section_data.insert(&pos, BlockDataInfo::create(BlockID::Water.id(), None));
                    }
                }
            }
        }
        return section_data;
    }
}
