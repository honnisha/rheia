pub mod blocks;
pub mod utils;
pub mod chunks;
pub mod world_generator;

pub const CHUNK_SIZE: u8 = 16_u8;
pub const CHUNK_SIZE_BOUNDARY: u32 = CHUNK_SIZE as u32 + 2;
pub const VERTICAL_SECTIONS: usize = 16;
