use std::{
    collections::BTreeMap,
    fs::{create_dir_all, read_dir, remove_file},
    io::{Seek, SeekFrom, Write},
};

use rusqlite::{Connection, DatabaseName, OptionalExtension, blob::ZeroBlob};

use crate::chunks::{
    chunk_data::{BlockIndexType, ChunkData},
    chunk_position::ChunkPosition,
};

use super::taits::{IWorldStorage, WorldInfo, WorldStorageSettings};

const SQL_TABLE_EXISTS: &str = "SELECT EXISTS(SELECT name FROM sqlite_master WHERE type='table' AND name='chunks');";

const SQL_CREATE_TABLE: &str =
    "CREATE TABLE IF NOT EXISTS chunks (id INTEGER PRIMARY KEY, x INTEGER, z INTEGER, sections_data BLOB)";
const SQL_CREATE_INDEX: &str = "CREATE INDEX coordinate_index ON chunks (x, z)";

const SQL_CREATE_INFO_TABLE: &str = "CREATE TABLE IF NOT EXISTS world_info (seed TEXT);";
const SQL_SET_SEED: &str = "INSERT INTO world_info (seed) VALUES (?1)";
const SQL_READ_SEED: &str = "SELECT seed FROM world_info;";

const SQL_SELECT_CHUNK_ID: &str = "SELECT id FROM chunks WHERE x=?1 AND z=?2;";
const SQL_INSERT_CHUNK: &str = "INSERT INTO chunks (x, z, sections_data) VALUES (?1, ?2, ?3);";
const SQL_UPDATE_CHUNK: &str = "UPDATE chunks SET sections_data = ?2 WHERE id=?1";

const SQL_CREATE_TABLE_IDS: &str =
    "CREATE TABLE IF NOT EXISTS world_block_ids (block_id INTEGER UNIQUE, block_slug STRING);";
const SQL_SELECT_IDS: &str = "SELECT block_id, block_slug FROM world_block_ids ORDER BY block_id;";
const SQL_INSERT_ID: &str = "INSERT INTO world_block_ids (block_id, block_slug) VALUES (?1, ?2);";

struct BlockId {
    block_id: BlockIndexType,
    block_slug: String,
}

pub struct SQLiteStorage {
    db: Connection,
    slug: String,
}

impl IWorldStorage for SQLiteStorage {
    type Error = String;
    type PrimaryKey = i64;

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

        let db = match Connection::open(path) {
            Ok(c) => c,
            Err(e) => return Err(format!("Database creation error: {}", e)),
        };

        let chunks_exists: bool = db.query_row(SQL_TABLE_EXISTS, [], |row| row.get(0)).unwrap();

        if !chunks_exists {
            if let Err(e) = db.execute(SQL_CREATE_TABLE, ()) {
                return Err(format!("World chunks creation error: &c{}", e));
            }

            if let Err(e) = db.execute(SQL_CREATE_INDEX, ()) {
                return Err(format!("World index create error: &c{}", e));
            }

            if let Err(e) = db.execute(SQL_CREATE_INFO_TABLE, ()) {
                return Err(format!("World info write error: &c{}", e));
            }

            if let Err(e) = db.execute(SQL_SET_SEED, (seed.to_string(),)) {
                return Err(format!("World seed save error: &c{}", e));
            }

            log::info!(target: "worlds", "World db &e\"{}\"&r created", path.to_str().unwrap());
        }

        Ok(Self { db, slug: world_slug })
    }

    fn has_chunk_data(&self, chunk_position: &ChunkPosition) -> Result<Option<Self::PrimaryKey>, String> {
        let chunks_exists: rusqlite::Result<i64> =
            self.db
                .query_row(SQL_SELECT_CHUNK_ID, (chunk_position.x, chunk_position.z), |row| {
                    row.get(0)
                });
        let r = match chunks_exists.optional() {
            Ok(r) => r,
            Err(e) => {
                return Err(format!("World seed save error: &c{}", e));
            }
        };
        return Ok(r);
    }

    fn load_chunk_data(&self, chunk_id: Self::PrimaryKey) -> Result<ChunkData, String> {
        let blob = self
            .db
            .blob_open(DatabaseName::Main, "chunks", "sections_data", chunk_id.clone(), true)
            .unwrap();
        let mut encoded = vec![0u8; blob.size() as usize];
        blob.read_at_exact(&mut encoded, 0).unwrap();

        let encoded_len = encoded.len();
        let sections = match ChunkData::decode_zip(encoded) {
            Ok(d) => d,
            Err(e) => {
                return Err(format!(
                    "Error: {} (encoded size:{} blob size: {})",
                    e,
                    encoded_len,
                    blob.size()
                ));
            }
        };
        Ok(sections)
    }

    fn save_chunk_data(&self, chunk_position: &ChunkPosition, data: &ChunkData) -> Result<Self::PrimaryKey, String> {
        let encoded = data.encode_zip();

        let id = match self.has_chunk_data(chunk_position) {
            Ok(id) => id,
            Err(e) => return Err(e),
        };
        let chunk_id = match id {
            Some(id) => {
                if let Err(e) = self.db.execute(SQL_UPDATE_CHUNK, (&id, ZeroBlob(encoded.len() as i32))) {
                    return Err(format!("Chunk update error: &c{}", e));
                }
                id
            }
            None => {
                if let Err(e) = self.db.execute(
                    SQL_INSERT_CHUNK,
                    (chunk_position.x, chunk_position.z, ZeroBlob(encoded.len() as i32)),
                ) {
                    return Err(format!("Chunk insert error: &c{}", e));
                }
                let id = self.db.last_insert_rowid();
                id
            }
        };

        let mut blob = self
            .db
            .blob_open(DatabaseName::Main, "chunks", "sections_data", chunk_id.clone(), false)
            .unwrap();
        let bytes_written = blob.write(encoded.as_slice()).unwrap();
        assert_eq!(encoded.len(), bytes_written);
        blob.seek(SeekFrom::Start(0)).unwrap();

        Ok(chunk_id)
    }

    fn scan_worlds(settings: &WorldStorageSettings) -> Result<Vec<WorldInfo>, String> {
        let mut worlds: Vec<WorldInfo> = Default::default();

        let mut folder_path = settings.get_data_path().clone();
        folder_path.push("worlds");
        if let Err(e) = std::fs::create_dir_all(folder_path.clone()) {
            return Err(format!(
                "&ccreate directory &4\"{}\"&r error: &c{}",
                folder_path.as_os_str().to_str().unwrap(),
                e
            ));
        }

        let paths = match read_dir(folder_path.clone()) {
            Ok(p) => p,
            Err(e) => {
                return Err(format!(
                    "&cread directory &4\"{}\"&r error: &c{}",
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
            let db = match Connection::open(path) {
                Ok(c) => c,
                Err(e) => return Err(format!("&cdatabase creation error: {}", e)),
            };
            let seed: String = match db.query_row(SQL_READ_SEED, [], |row| row.get(0)) {
                Ok(s) => s,
                Err(e) => return Err(format!("&cworld &4\"{}\"&r error seed read: &c{}", path, e)),
            };
            worlds.push(WorldInfo {
                slug: filename.replace(".db", ""),
                seed: seed.parse::<u64>().unwrap(),
            });
        }

        Ok(worlds)
    }

    fn delete(&self, settings: &WorldStorageSettings) -> Result<(), String> {
        let mut path = settings.get_data_path().clone();
        path.push("worlds");
        path.push(format!("{}.db", self.slug));
        if let Err(e) = remove_file(path.clone()) {
            return Err(format!(
                "world delete &e\"{}\"&r error: {}",
                path.as_os_str().to_str().unwrap(),
                e
            ));
        };
        log::info!(target: "worlds", "World db &e\"{}\"&r deleted", path.to_str().unwrap());
        Ok(())
    }

    fn validate_block_id_map(
        world_slug: String,
        settings: &WorldStorageSettings,
        block_id_map: &BTreeMap<BlockIndexType, String>,
    ) -> Result<(), String> {
        let mut path = settings.get_data_path().clone();
        path.push("worlds");
        path.push(format!("{}.db", world_slug));
        let path = path.as_os_str();

        let db = match Connection::open(path) {
            Ok(c) => c,
            Err(e) => return Err(format!("Database creation error: {}", e)),
        };

        if let Err(e) = db.execute(SQL_CREATE_TABLE_IDS, ()) {
            return Err(format!("World block ids table create error: &c{}", e));
        }

        let mut stmt = db.prepare(SQL_SELECT_IDS).unwrap();
        let ids_result = stmt
            .query_map([], |row| {
                Ok(BlockId {
                    block_id: row.get(0).unwrap(),
                    block_slug: row.get(1).unwrap(),
                })
            })
            .unwrap();

        let mut existing_blocks: Vec<String> = Default::default();
        for block_row in ids_result {
            let block_row = block_row.unwrap();

            // Check that saved id map contains all block from world
            let mut block_exists = false;
            for (block_id, block_slug) in block_id_map.iter() {
                if *block_slug == block_row.block_slug {
                    if *block_id != block_row.block_id {
                        return Err(format!(
                            "&cblock &4\"{}\"&c id is not match; world_id:{} saved_id:{}",
                            block_slug, block_row.block_id, block_id
                        ));
                    }
                    block_exists = true;
                }
            }
            if !block_exists {
                return Err(format!(
                    "&cblock &4\"{}\"&c doesn't exists in resources",
                    block_row.block_slug
                ));
            }
            existing_blocks.push(block_row.block_slug.clone());
        }

        // Check that all blocks exists inside world and write if not
        for (block_id, block_slug) in block_id_map.iter() {
            if !existing_blocks.contains(&block_slug) {
                // Block id is not exists in the world;
                if let Err(e) = db.execute(SQL_INSERT_ID, (block_id.clone(), block_slug.clone())) {
                    return Err(format!(
                        "Block id #{} \"{}\" insert error: &c{}",
                        block_id, block_slug, e
                    ));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::{
        chunks::{chunk_data::ChunkData, chunk_position::ChunkPosition},
        world_generator::{
            default::{WorldGenerator, WorldGeneratorSettings},
            traits::IWorldGenerator,
        },
        worlds_storage::{
            sqlite_storage::SQLiteStorage,
            taits::{IWorldStorage, WorldStorageSettings},
        },
    };

    fn generate_chunk(seed: u64, chunk_position: &ChunkPosition) -> ChunkData {
        let generator = WorldGenerator::create(Some(seed), WorldGeneratorSettings::default()).unwrap();
        generator.generate_chunk_data(&chunk_position)
    }

    #[test]
    fn test_worlds() {
        let data_path = env::current_dir().unwrap().clone();
        let settings = WorldStorageSettings::create(data_path);
        let storage = SQLiteStorage::create("tests".to_string(), 1, &settings).unwrap();

        let chunk_position = ChunkPosition::new(0, 0);
        let sections = generate_chunk(1, &chunk_position);

        // Confirm that there is not chunk
        assert_eq!(storage.has_chunk_data(&chunk_position).unwrap(), None);

        // Save new chunk
        let chunk_id = storage.save_chunk_data(&chunk_position, &sections).unwrap();
        let has_chunk_id = storage.has_chunk_data(&chunk_position).unwrap().unwrap();
        assert_eq!(has_chunk_id, chunk_id);

        // Save new chunk
        let sections = generate_chunk(2, &chunk_position);
        let updated_chunk_id = storage.save_chunk_data(&chunk_position, &sections).unwrap();
        assert_eq!(has_chunk_id, updated_chunk_id);

        let loaded_sections = storage.load_chunk_data(has_chunk_id).unwrap();
        assert_eq!(loaded_sections.get(0).unwrap().len(), sections.get(0).unwrap().len());

        storage.delete(&settings).unwrap();
    }
}
