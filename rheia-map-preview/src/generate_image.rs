const SIZE: i64 = 16;
pub const IMAGE_SIZE: u32 = (CHUNK_SIZE as f32 * SIZE as f32) as u32;

use common::{
    chunks::{block_position::BlockPosition, chunk_data::{BlockDataInfo, ChunkData}, chunk_position::ChunkPosition}, default_blocks_ids::BlockID, world_generator::{
        default::{WorldGenerator, WorldGeneratorSettings},
        traits::IWorldGenerator,
    }, CHUNK_SIZE, VERTICAL_SECTIONS
};
use image::{ImageBuffer, Rgb};


pub fn generate_map_image(seed: u64) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, String> {
    let settings: WorldGeneratorSettings = match serde_yaml::from_str("") {
        Ok(g) => g,
        Err(e) => return Err(e.to_string()),
    };
    let generator = match WorldGenerator::create(Some(seed), settings) {
        Ok(g) => g,
        Err(e) => return Err(e),
    };

    let mut imgbuf = ImageBuffer::new(IMAGE_SIZE, IMAGE_SIZE);
    imgbuf.put_pixel(0, 0, Rgb([0_u8, 0_u8, 0_u8]));

    for chunk_x in 0..SIZE {
        for chunk_y in 0..SIZE {
            let chunk_position = ChunkPosition::new(chunk_x, chunk_y);
            let chunk_data = generator.generate_chunk_data(&chunk_position);
            generate_chunk(&chunk_data, &chunk_position, &mut imgbuf);
        }
    }
    Ok(imgbuf)
}

fn generate_chunk(chunk_data: &ChunkData, chunk_position: &ChunkPosition, imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            let x_map = x as f32 + (chunk_position.x as f32 * CHUNK_SIZE as f32);
            let z_map = z as f32 + (chunk_position.z as f32 * CHUNK_SIZE as f32);

            let mut last_block_info: Option<BlockDataInfo>= None;

            for y in 0..(VERTICAL_SECTIONS * CHUNK_SIZE as usize) {
                let block_position = BlockPosition::new(x as i64, y as i64, z as i64);
                match chunk_data.get_block_info(&block_position) {
                    Some(block_info) => {
                        last_block_info = Some(block_info.clone());
                    },
                    None => {
                        let pixel = imgbuf.get_pixel_mut(x_map as u32, z_map as u32);

                        let Some(last_block_info) = last_block_info else {
                            break;
                        };

                        // TODO: colors
                        let Some(_block_info) = BlockID::from_id(&last_block_info.get_id()) else {
                            break;
                        };

                        let value = (y * 2) as u8;
                        *pixel = Rgb([value, value, value]);
                        break;
                    },
                }
            }
        }
    }
}
