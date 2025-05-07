use std::fs::{create_dir_all, read_dir};

use rusqlite::{Connection, OptionalExtension};

use crate::chunks::chunk_position::ChunkPosition;

use super::taits::{ChunkData, IWorldStorage, WorldInfo, WorldStorageSettings};

pub struct SQLLiteStorage {
    conn: Connection,
}

const SQL_TABLE_EXISTS: &str = "SELECT name FROM sqlite_master WHERE type='table' AND name='chunks';";

const SQL_CREATE_TABLE: &str = "CREATE TABLE IF NOT EXISTS chunks (x integer, z integer, sections_data blob)";
const SQL_CREATE_INDEX: &str = "CREATE INDEX coordinate_index ON chunks (x, z)";

const SQL_CREATE_INFO: &str = "CREATE TABLE IF NOT EXISTS world_info (seed integer)";
const SQL_SET_SEED: &str = "INSERT INTO world_info (seed) VALUES (?1)";
const SQL_READ_SEED: &str = "SELECT seed FROM world_info";

impl IWorldStorage for SQLLiteStorage {
    type Error = String;

    fn create(world_slug: String, seed: u64, settings: &WorldStorageSettings) -> Result<Self, String> {
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

        let res: rusqlite::Result<String> = conn.query_row(SQL_TABLE_EXISTS, [], |row| row.get(0));

        if res.optional().unwrap().is_none() {
            if let Err(e) = conn.execute(SQL_CREATE_TABLE, ()) {
                return Err(format!("World chunks creation error: &c{}", e));
            }

            if let Err(e) = conn.execute(SQL_CREATE_INDEX, ()) {
                return Err(format!("World index create error: &c{}", e));
            }

            if let Err(e) = conn.execute(SQL_CREATE_INFO, ()) {
                return Err(format!("World info write error: &c{}", e));
            }

            if let Err(e) = conn.execute(SQL_SET_SEED, (seed,)) {
                return Err(format!("World seed save error: &c{}", e));
            }

            log::info!(target: "worlds", "World db &e\"{}\"&r created", path.to_str().unwrap());
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

    fn scan_worlds(settings: &WorldStorageSettings) -> Result<Vec<WorldInfo>, String> {
        let mut worlds: Vec<WorldInfo> = Default::default();

        let mut folder_path = settings.get_data_path().clone();
        folder_path.push("worlds");
        let paths = match read_dir(folder_path.clone()) {
            Ok(p) => p,
            Err(e) => {
                return Err(format!(
                    "read directory &e\"{}\"&r error: &c{}",
                    folder_path.as_os_str().to_str().unwrap(),
                    e
                ));
            }
        };
        for path in paths {
            let path = path.unwrap().path();
            let filename = path.file_name().unwrap().to_str().unwrap();
            let path = path.as_os_str().to_str().unwrap();
            if !path.ends_with(".db") {
                continue;
            }
            let conn = match Connection::open(path) {
                Ok(c) => c,
                Err(e) => return Err(format!("Database creation error: &c{}", e)),
            };
            let seed: u64 = match conn.query_row(SQL_READ_SEED, [], |row| row.get(0)) {
                Ok(s) => s,
                Err(e) => return Err(format!("World &e\"{}\"&r error seed read: &c{}", path, e)),
            };
            worlds.push(WorldInfo {
                slug: filename.replace(".db", ""),
                seed,
            });
        }

        Ok(worlds)
    }
}
