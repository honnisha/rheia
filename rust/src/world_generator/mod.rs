use bracket_noise::prelude::*;
use ndshape::ConstShape;

use crate::{chunks::block_info::BlockInfo, mesh::mesh_generator::ChunkShape};

pub struct WorldGenerator {
    noise: FastNoise,
}

impl WorldGenerator {
    pub fn new(seeded: u64) -> Self {
        let mut noise = FastNoise::seeded(seeded);
        noise.set_noise_type(NoiseType::SimplexFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(5);
        noise.set_fractal_gain(0.6);
        noise.set_fractal_lacunarity(0.1);
        noise.set_frequency(1.5);

        WorldGenerator { noise: noise }
    }

    pub fn generate_chunk_data(
        &mut self,
        chunk_data: &mut [BlockInfo; 4096],
        chunk_position: &[i32; 3],
    ) {
        for x in 0_u32..16_u32 {
            for z in 0_u32..16_u32 {

                let x_map = (x as f32 + (chunk_position[0] as f32 * 16_f32)) / 100.0;
                let z_map = (z as f32 + (chunk_position[2] as f32 * 16_f32)) / 100.0;
                let height = self.noise.get_noise(x_map, z_map) * 15_f32 + 10_f32;

                for y in 0_u32..16_u32 {
                    let i = ChunkShape::linearize([x, y, z]);
                    assert!(i < ChunkShape::SIZE, "Generate chunk data overflow array length; dimentions:{:?} current:{:?}", ChunkShape::ARRAY, [x, y, z]);

                    let y_global = y as f32 + (chunk_position[1] as f32 * 16_f32);
                    if height > y_global {
                        chunk_data[i as usize] = BlockInfo::new(1);
                    }
                }
            }
        }
    }
}
