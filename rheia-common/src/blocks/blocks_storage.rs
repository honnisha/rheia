use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::{block_type_info::BlockTypeInfo, voxel_visibility::VoxelVisibility};

#[derive(Debug, Clone, Copy, Eq, EnumIter, Serialize, Deserialize)]
#[repr(u16)]
pub enum BlockType {
    Air,
    Stone,
    GrassBlock,
}

const BLOCK_AIR: BlockTypeInfo = BlockTypeInfo::new_empty();
const BLOCK_STONE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("stone.png");
const BLOCK_GRASS_BLOCK: BlockTypeInfo = BlockTypeInfo {
    voxel_visibility: VoxelVisibility::Opaque,
    top_texture: Some("grass_top.png"),
    side_texture: Some("grass_block_side.png"),
    bottom_texture: Some("dirt.png"),
};

pub fn get_block_type_info(block_type: &BlockType) -> Option<&'static BlockTypeInfo> {
    match block_type {
        BlockType::Air => Some(&BLOCK_AIR),
        BlockType::Stone => Some(&BLOCK_STONE),
        BlockType::GrassBlock => Some(&BLOCK_GRASS_BLOCK),
    }
}

impl PartialEq for BlockType {
    fn eq(&self, other: &BlockType) -> bool {
        *self as u8 == *other as u8
    }
}

impl BlockType {
    pub fn to_iter() -> BlockTypeIter {
        BlockType::iter()
    }
}
