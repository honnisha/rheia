use bracket_noise::prelude::*;
use common::{
    blocks::{block_info::BlockInfo, blocks_storage::BlockType},
    CHUNK_SIZE, network::ChunkDataType,
};

use super::chunks::chunks_map::ChunkPosition;

pub struct WorldGenerator {
    noise: FastNoise,
}

impl WorldGenerator {
    pub fn new(seed: u64) -> Self {
        let mut noise = FastNoise::seeded(seed);
        noise.set_noise_type(NoiseType::PerlinFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(5);
        noise.set_fractal_gain(0.6);
        noise.set_fractal_lacunarity(1.5);
        noise.set_frequency(2.0);

        WorldGenerator { noise: noise }
    }

    pub fn generate_chunk_data(
        &self,
        chunk_data: &mut ChunkDataType,
        chunk_position: &ChunkPosition,
        vertical_index: usize,
    ) -> bool {
        //let now = Instant::now();
        let mut has_any_block = false;
        for x in 0_u8..(CHUNK_SIZE as u8) {
            for z in 0_u8..(CHUNK_SIZE as u8) {
                let x_map = (x as f32 + (chunk_position.x as f32 * CHUNK_SIZE as f32)) / 150.0;
                let z_map = (z as f32 + (chunk_position.z as f32 * CHUNK_SIZE as f32)) / 150.0;
                let height = self.noise.get_noise(x_map, z_map) * 40_f32 + 20_f32;

                //godot_print!("x{} z:{} height:{}", x, z, height);
                for y in 0_u8..(CHUNK_SIZE as u8) {
                    let pos = [x, y, z];

                    let y_global = y as f32 + (vertical_index as f32 * CHUNK_SIZE as f32);

                    if height > y_global {
                        has_any_block = true;
                        chunk_data.insert(pos, BlockInfo::new(BlockType::GrassBlock));
                    }
                }
            }
        }
        //println!("Chunk {:?} data generated in {:.2?}", chunk_pos, now.elapsed());
        return has_any_block;
    }
}
