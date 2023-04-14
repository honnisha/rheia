use crate::world::chunks::chunk_info::{ChunkData, ChunkShape};
use crate::{world::blocks::blocks_storage::BlockType, world::chunks::block_info::BlockInfo};
use bracket_noise::prelude::*;
use ndshape::ConstShape;

use super::chunks::chunk_info::CHUNK_SIZE;

pub struct WorldGenerator {
    noise: FastNoise,
}

impl WorldGenerator {
    pub fn new(seeded: u64) -> Self {
        let mut noise = FastNoise::seeded(seeded);
        noise.set_noise_type(NoiseType::PerlinFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(5);
        noise.set_fractal_gain(0.6);
        noise.set_fractal_lacunarity(1.5);
        noise.set_frequency(2.0);

        WorldGenerator { noise: noise }
    }

    pub fn generate_chunk_data(&self, chunk_data: &mut ChunkData, chunk_pos: &[i32; 3]) -> bool {
        //let now = Instant::now();
        let mut has_any_block = false;
        for x in 0_u32..(CHUNK_SIZE as u32) {
            for z in 0_u32..(CHUNK_SIZE as u32) {
                let x_map = (x as f32 + (chunk_pos[0] as f32 * CHUNK_SIZE as f32)) / 150.0;
                let z_map = (z as f32 + (chunk_pos[2] as f32 * CHUNK_SIZE as f32)) / 150.0;
                let height = self.noise.get_noise(x_map, z_map) * 40_f32 + 20_f32;

                //godot_print!("x{} z:{} height:{}", x, z, height);
                for y in 0_u32..(CHUNK_SIZE as u32) {
                    let pos = [x, y, z];
                    let i = ChunkShape::linearize(pos);
                    assert!(
                        i < ChunkShape::SIZE,
                        "Generate chunk data overflow array length; dimentions:{:?} current:{:?}",
                        ChunkShape::ARRAY,
                        pos
                    );

                    let y_global = y as f32 + (chunk_pos[1] as f32 * CHUNK_SIZE as f32);

                    if height > y_global {
                        has_any_block = true;
                        chunk_data[i as usize] = BlockInfo::new(BlockType::GrassBlock);
                    }
                }
            }
        }
        //println!("Chunk {:?} data generated in {:.2?}", chunk_pos, now.elapsed());
        return has_any_block;
    }
}
