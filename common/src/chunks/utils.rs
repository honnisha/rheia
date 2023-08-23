use std::collections::HashMap;

use arrayvec::ArrayVec;
use serde::{Deserialize, Serialize};

use crate::{blocks::block_info::BlockInfo, network::messages::{ChunkDataType, NetworkSectionsType}, VERTICAL_SECTIONS};

use super::block_position::ChunkBlockPosition;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PacketChunkSectionData {
    pallete: Vec<BlockInfo>,
    block_indexes: HashMap<ChunkBlockPosition, u32>,
}

impl PacketChunkSectionData {
    pub fn new(chunk_data: &mut ChunkDataType) -> Self {
        let mut data: Self = Default::default();
        for (pos, block_info) in chunk_data.drain() {
            data.store_block(pos, block_info);
        }
        data
    }

    fn store_type(&mut self, block_info: BlockInfo) -> u32 {
        if self.pallete.contains(&block_info) {
            let index = self.pallete.iter().position(|&r| r == block_info).unwrap();
            index as u32
        } else {
            self.pallete.push(block_info.clone());
            (self.pallete.len() - 1) as u32
        }
    }

    fn store_block(&mut self, pos: ChunkBlockPosition, block_info: BlockInfo) {
        let index = self.store_type(block_info);
        self.block_indexes.insert(pos, index);
    }

    pub fn unpack(&mut self) -> ChunkDataType {
        let mut data: ChunkDataType = Default::default();
        for (pos, index) in self.block_indexes.drain() {
            data.insert(pos, self.pallete.get(index as usize).unwrap().clone());
        }
        data
    }
}

pub type SectionsData = [Box<ChunkDataType>; VERTICAL_SECTIONS];

pub fn unpack_network_sectioins(network_data: &mut NetworkSectionsType) -> SectionsData {
    let mut result: ArrayVec<Box<ChunkDataType>, VERTICAL_SECTIONS> = Default::default();
    for packet_section in network_data {
        result.push(Box::new(packet_section.unpack()));
    }
    result.into_inner().unwrap()
}
