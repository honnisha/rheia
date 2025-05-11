use std::collections::HashMap;

use crate::{
    blocks::block_info::BlockInfo,
    chunks::{
        block_position::ChunkBlockPosition,
        chunk_data::{ChunkData, ChunkSectionDataType},
        chunk_position::ChunkPosition,
    },
    CHUNK_SIZE, VERTICAL_SECTIONS,
};
use bracket_lib::random::RandomNumberGenerator;
use bracket_noise::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct WorldGeneratorSettings {
    fractal_type: Option<i32>,
    fractal_gain: Option<f32>,
    fractal_lacunarity: Option<f32>,
    frequency: Option<f32>,
}

pub struct WorldGenerator {
    noise: FastNoise,
    _settings: WorldGeneratorSettings,
}

impl WorldGenerator {
    pub fn create(seed: Option<u64>, settings: WorldGeneratorSettings) -> Result<Self, String> {
        let seed = match seed {
            Some(s) => s,
            None => {
                let mut rng = RandomNumberGenerator::new();
                rng.next_u64()
            }
        };

        let mut noise = FastNoise::seeded(seed);
        noise.set_noise_type(NoiseType::PerlinFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(settings.fractal_type.unwrap_or(1));
        noise.set_fractal_gain(settings.fractal_gain.unwrap_or(0.6));
        noise.set_fractal_lacunarity(settings.fractal_lacunarity.unwrap_or(1.5));
        noise.set_frequency(settings.frequency.unwrap_or(2.0));

        Ok(Self {
            noise,
            _settings: settings,
        })
    }

    pub fn generate_chunk_data(&self, chunk_position: &ChunkPosition) -> ChunkData {
        let mut chunk_data: ChunkData = Default::default();
        for y in 0..VERTICAL_SECTIONS {
            let chunk_section = self.generate_section_data(&chunk_position, y);
            chunk_data.push_section(Box::new(chunk_section));
        }
        chunk_data
    }

    fn generate_section_data(&self, chunk_position: &ChunkPosition, vertical_index: usize) -> ChunkSectionDataType {
        let mut section_data: ChunkSectionDataType = HashMap::new();

        for x in 0_u8..(CHUNK_SIZE as u8) {
            for z in 0_u8..(CHUNK_SIZE as u8) {
                let x_map = (x as f32 + (chunk_position.x as f32 * CHUNK_SIZE as f32)) / 150.0;
                let z_map = (z as f32 + (chunk_position.z as f32 * CHUNK_SIZE as f32)) / 150.0;
                let height = self.noise.get_noise(x_map, z_map) * 40_f32 + 20_f32;

                //godot_print!("x{} z:{} height:{}", x, z, height);
                for y in 0_u8..(CHUNK_SIZE as u8) {
                    let pos = ChunkBlockPosition::new(x, y, z);

                    let y_global = y as f32 + (vertical_index as f32 * CHUNK_SIZE as f32);

                    if height > y_global {
                        section_data.insert(pos, BlockInfo::create(1, None)); // GrassBlock
                    }

                    if x == 0 && y_global as f32 == 24.0 && z == 0 {
                        section_data.insert(pos, BlockInfo::create(1, None)); // GrassBlock
                    }
                }
            }
        }
        return section_data;
    }
}
