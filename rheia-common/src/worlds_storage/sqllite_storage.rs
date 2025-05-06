use std::fs::create_dir_all;

use rusqlite::Connection;

use crate::chunks::chunk_position::ChunkPosition;

use super::taits::{ChunkData, IWorldStorage, WorldInfo, WorldStorageSettings};

pub struct SQLLiteStorage {
    conn: Connection,
}

const SQL_CREATE_TABLE: &str = "CREATE TABLE IF NOT EXISTS chunks (x integer, z integer, sections_data blob)";
const SQL_CREATE_INDEX: &str = "CREATE INDEX coordinate_index ON MyTable (x, y)";

impl IWorldStorage for SQLLiteStorage {
    type Error = String;

    fn create(world_slug: String, settings: &WorldStorageSettings) -> Result<Self, String> {
        let mut path = settings.get_data_path().clone();
        path.push("worlds");

        if create_dir_all(&path).is_err() {
            return Err(format!(
                "Unable to create dir \"{}\"",
                path.as_os_str().to_str().unwrap()
            ));
        }

        path.push(format!("{}.db", world_slug));
        let path = path.as_os_str();

        let conn = match Connection::open(path) {
            Ok(c) => c,
            Err(e) => return Err(format!("Database creation error: {}", e)),
        };

        let res = conn.execute(SQL_CREATE_TABLE, ()).unwrap();

        if res > 0 {
            conn.execute(SQL_CREATE_INDEX, ()).unwrap();
            log::info!(target: "world_storage", "World db \"{}\" created", path.to_str().unwrap());
        }

        Ok(Self { conn })
    }

    fn has_chunk_data(&self, _chunk_position: &ChunkPosition) -> bool {
        false
    }

    fn load_chunk_data(&self, _chunk_position: &ChunkPosition) -> ChunkData {
        unimplemented!()
    }

    fn save_chunk_data(&self, _chunk_position: &ChunkPosition, _data: &ChunkData) {}

    fn scan_worlds(settings: &WorldStorageSettings) -> Vec<WorldInfo> {
        let worlds: Vec<WorldInfo> = Default::default();
        worlds
    }
}
