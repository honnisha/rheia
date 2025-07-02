use worlds_storage::sqlite_storage::SQLiteStorage;

pub mod blocks;
pub mod chunks;
pub mod utils;
pub mod world_generator;
pub mod worlds_storage;
pub mod default_resources;
pub mod default_blocks;
pub mod default_blocks_ids;
pub mod commands;

pub type WorldStorageManager = SQLiteStorage;

pub const CHUNK_SIZE: u8 = 16_u8;
pub const CHUNK_SIZE_BOUNDARY: u32 = CHUNK_SIZE as u32 + 2;
pub const VERTICAL_SECTIONS: usize = 16;
