use strum_macros::EnumIter;

use crate::utils::block_mesh::{Voxel, VoxelVisibility};

use super::block_type_info::BlockTypeInfo;

#[derive(Debug, Clone, Copy, Eq, EnumIter)]
#[repr(u8)]
pub enum BlockType {
    Air,
    GrassBlock,
    Stone,
    Water,
}

impl BlockType {
    pub fn get_block_type_info(&self) -> Option<&'static BlockTypeInfo> {
        get_block_type_info(self)
    }
}

impl Voxel for BlockType {
    fn get_visibility(&self) -> VoxelVisibility {
        match get_block_type_info(self) {
            Some(t) => t.voxel_visibility.clone(),
            None => VoxelVisibility::Empty,
        }
    }
    fn get_type(&self) -> &BlockType {
        return self;
    }
}

impl PartialEq for BlockType {
    fn eq(&self, other: &BlockType) -> bool {
        *self as u8 == *other as u8
    }
}

const BLOCK_AIR: BlockTypeInfo = BlockTypeInfo {
    voxel_visibility: VoxelVisibility::Empty,
    top_texture: None,
    side_texture: None,
    bottom_texture: None,
};
const BLOCK_GRASS_BLOCK: BlockTypeInfo = BlockTypeInfo {
    voxel_visibility: VoxelVisibility::Opaque,
    top_texture: Some("grass_top.png"),
    side_texture: Some("grass_block_side.png"),
    bottom_texture: Some("dirt.png"),
};
const BLOCK_STONE: BlockTypeInfo = BlockTypeInfo {
    voxel_visibility: VoxelVisibility::Opaque,
    top_texture: Some("stone.png"),
    side_texture: Some("stone.png"),
    bottom_texture: Some("stone.png"),
};
const BLOCK_WATER: BlockTypeInfo = BlockTypeInfo {
    voxel_visibility: VoxelVisibility::Translucent,
    top_texture: Some("water_overlay.png"),
    side_texture: Some("water_overlay.png"),
    bottom_texture: Some("water_overlay.png"),
};

fn get_block_type_info(block_type: &BlockType) -> Option<&'static BlockTypeInfo> {
    match block_type {
        BlockType::Air => Some(&BLOCK_AIR),
        BlockType::GrassBlock => Some(&BLOCK_GRASS_BLOCK),
        BlockType::Stone => Some(&BLOCK_STONE),
        BlockType::Water => Some(&BLOCK_WATER),
    }
}
