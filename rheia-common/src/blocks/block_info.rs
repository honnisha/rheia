use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    chunks::{chunk_data::BlockIndexType, rotation::Rotation},
    default_blocks_ids::{BlockID, CUSTOM_BLOCK_ID_START},
};

use super::block_type::BlockType;

#[derive(Debug, Clone, Copy, Eq, Serialize, Deserialize, PartialEq)]
pub enum BlockFace {
    East,
    North,
    South,
    West,
}

impl Default for BlockFace {
    fn default() -> Self {
        BlockFace::South
    }
}

impl BlockFace {
    pub fn rotate_left(&self) -> BlockFace {
        match *self {
            BlockFace::East => BlockFace::South,
            BlockFace::North => BlockFace::East,
            BlockFace::South => BlockFace::West,
            BlockFace::West => BlockFace::North,
        }
    }

    pub fn rotate_right(&self) -> BlockFace {
        match *self {
            BlockFace::East => BlockFace::North,
            BlockFace::North => BlockFace::West,
            BlockFace::South => BlockFace::East,
            BlockFace::West => BlockFace::South,
        }
    }

    pub fn get_rotation(&self) -> Rotation {
        match *self {
            BlockFace::East => Rotation::new(0.0, 270.0),
            BlockFace::North => Rotation::new(0.0, 180.0),
            BlockFace::South => Rotation::new(0.0, 0.0),
            BlockFace::West => Rotation::new(0.0, 90.0),
        }
    }
}

fn generate_block_id(block_type: &BlockType, last_id: BlockIndexType) -> BlockIndexType {
    let Some(id) = BlockID::from_string(&block_type.get_slug()) else {
        return last_id.max(CUSTOM_BLOCK_ID_START) + 1;
    };
    id.id()
}

pub fn generate_block_id_map<'a>(
    block_id_map: &mut BTreeMap<BlockIndexType, String>,
    blocks_iter: impl Iterator<Item = &'a BlockType>,
) -> Result<(), String> {
    for block_type in blocks_iter {
        let mut existed = false;
        for (block_id, block_slug) in block_id_map.iter() {
            if block_slug == block_type.get_slug() {
                existed = true;

                if let Some(id) = BlockID::from_string(&block_type.get_slug()) {
                    // If stored id is not equal to hardcoded
                    if *block_id != id.id() {
                        return Err(format!(
                            "block \"{}\" stored id:{} is not equal to hardcoded id:{}",
                            block_type.get_slug(),
                            block_id,
                            id.id()
                        ));
                    }
                }
            }
        }

        // Get last id
        let mut last_id: BlockIndexType = 0;
        for (block_id, _block_slug) in block_id_map.iter() {
            last_id = *block_id.max(&last_id);
        }

        if !existed {
            let block_id = generate_block_id(&block_type, last_id);
            block_id_map.insert(block_id, block_type.get_slug().clone());
        }
    }
    Ok(())
}
