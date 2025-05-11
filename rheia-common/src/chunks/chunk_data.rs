use crate::{blocks::block_info::BlockInfo, VERTICAL_SECTIONS};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::block_position::{BlockPosition, ChunkBlockPosition};

pub type ChunkSectionDataType = HashMap<ChunkBlockPosition, BlockInfo>;
type SectionData = Box<ChunkSectionDataType>;
type ChunkDataType = Vec<Box<ChunkSectionDataType>>;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct ChunkData {
    data: ChunkDataType,
}

impl ChunkData {
    pub fn encode(&self) -> Vec<u8> {
        let encoded = bincode::serialize(&self).unwrap();
        encoded
    }

    pub fn get(&self, index: usize) -> Option<&SectionData> {
        self.data.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut SectionData> {
        self.data.get_mut(index)
    }

    pub fn change_block(&mut self, section: u32, pos: ChunkBlockPosition, block: Option<BlockInfo>) {
        if section > VERTICAL_SECTIONS as u32 {
            panic!("Tried to change block in section {section} more than max {VERTICAL_SECTIONS}");
        }

        match block {
            Some(i) => {
                self.data[section as usize].insert(pos, i);
            }
            None => {
                self.data[section as usize].remove(&pos);
            }
        }
    }

    pub fn get_block_info(&self, block_position: &BlockPosition) -> Option<BlockInfo> {
        let (section, block_position) = block_position.get_block_position();
        match self.data[section as usize].get(&block_position) {
            Some(b) => Some(b.clone()),
            None => None,
        }
    }

    pub fn push_section(&mut self, data: SectionData) {
        if self.data.len() >= VERTICAL_SECTIONS {
            panic!("Tried to insert sections more than max {VERTICAL_SECTIONS}");
        }
        self.data.push(data);
    }

    pub fn decode(encoded: Vec<u8>) -> Result<Self, String> {
        let chunk_data: Self = match bincode::deserialize(&encoded) {
            Ok(d) => d,
            Err(e) => return Err(format!("Decode chunk error: &c{} ", e)),
        };
        Ok(chunk_data)
    }
}
