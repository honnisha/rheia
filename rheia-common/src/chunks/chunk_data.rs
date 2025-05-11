use crate::{blocks::block_info::BlockInfo, VERTICAL_SECTIONS};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{Read, Write},
};
use zip::{CompressionMethod, DateTime};

use super::block_position::{BlockPosition, ChunkBlockPosition};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct ChunkSectionData {
    data: HashMap<u16, BlockInfo>,
}

const COMPRESS: CompressionMethod = CompressionMethod::Bzip2;

impl ChunkSectionData {
    pub fn change(&mut self, pos: &ChunkBlockPosition, block: Option<BlockInfo>) {
        match block {
            Some(i) => {
                self.data.insert(pos.linearize(), i);
            }
            None => {
                self.data.remove(&pos.linearize());
            }
        }
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, u16, BlockInfo> {
        self.data.iter()
    }

    pub fn insert(&mut self, pos: &ChunkBlockPosition, block: BlockInfo) -> Option<BlockInfo> {
        self.data.insert(pos.linearize(), block)
    }

    pub fn get(&self, pos: &ChunkBlockPosition) -> Option<&BlockInfo> {
        self.data.get(&pos.linearize())
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct ChunkData {
    data: Vec<Box<ChunkSectionData>>,
}

impl ChunkData {
    pub fn encode_zip(&self) -> Vec<u8> {
        let mut archive_data: Vec<u8> = Default::default();

        let buff = std::io::Cursor::new(&mut archive_data);
        let mut writer = zip::ZipWriter::new(buff);

        let options = zip::write::SimpleFileOptions::default()
            .compression_method(COMPRESS)
            .last_modified_time(DateTime::default());

        writer.start_file("data", options).unwrap();
        writer.write_all(&self.encode()).unwrap();
        writer.finish().unwrap();
        archive_data
    }

    pub fn encode(&self) -> Vec<u8> {
        let encoded = bincode::serialize(&self).unwrap();
        encoded
    }

    pub fn decode_zip(data: Vec<u8>) -> Result<Self, String> {
        let file = std::io::Cursor::new(&data);
        let mut zip = zip::ZipArchive::new(file).unwrap();

        let mut archive_file_data = Vec::new();

        for i in 0..zip.len() {
            let archive_file = zip.by_index(i).unwrap();
            for i in archive_file.bytes() {
                archive_file_data.push(i.unwrap());
            }
            break;
        }
        ChunkData::decode(archive_file_data)
    }

    pub fn decode(encoded: Vec<u8>) -> Result<Self, String> {
        let chunk_data: Self = match bincode::deserialize(&encoded) {
            Ok(d) => d,
            Err(e) => return Err(format!("Decode chunk error: &c{} ", e)),
        };
        Ok(chunk_data)
    }

    pub fn change_block(&mut self, section: u32, pos: &ChunkBlockPosition, block: Option<BlockInfo>) {
        if section > VERTICAL_SECTIONS as u32 {
            panic!("Tried to change block in section {section} more than max {VERTICAL_SECTIONS}");
        }

        self.data[section as usize].change(&pos, block);
    }

    pub fn get(&self, index: usize) -> Option<&Box<ChunkSectionData>> {
        self.data.get(index)
    }

    pub fn get_block_info(&self, block_position: &BlockPosition) -> Option<BlockInfo> {
        let (section, block_position) = block_position.get_block_position();
        match self.data[section as usize].get(&block_position) {
            Some(b) => Some(b.clone()),
            None => None,
        }
    }

    pub fn push_section(&mut self, data: ChunkSectionData) {
        if self.data.len() >= VERTICAL_SECTIONS {
            panic!("Tried to insert sections more than max {VERTICAL_SECTIONS}");
        }
        self.data.push(Box::new(data));
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        chunks::{chunk_data::ChunkData, chunk_position::ChunkPosition},
        world_generator::default::{WorldGenerator, WorldGeneratorSettings},
    };

    #[test]
    fn test_chunks_data() {
        let generator = WorldGenerator::create(Some(1), WorldGeneratorSettings::default()).unwrap();

        let chunk_position = ChunkPosition::new(0, 0);
        let chunk_data = generator.generate_chunk_data(&chunk_position);

        let encoded = chunk_data.encode();
        assert_eq!(encoded.len(), 23321);

        let encoded = chunk_data.encode_zip();
        assert!(encoded.len() < 7500);

        let decoded_chunk_data = ChunkData::decode_zip(encoded).unwrap();
        assert_eq!(
            chunk_data.get(0).unwrap().len(),
            decoded_chunk_data.get(0).unwrap().len()
        );
    }
}
