use crate::blocks::{
    block_info::BlockIndexType,
    block_type::{BlockContent, BlockType},
    voxel_visibility::VoxelVisibility,
};
use lazy_static::lazy_static;
use std::collections::HashMap;


lazy_static! {
    pub static ref DEFAULT_BLOCKS: HashMap<BlockIndexType, BlockType> = {
        let mut m = HashMap::new();
        m.insert(
            1,
            BlockType::new(
                VoxelVisibility::Opaque,
                BlockContent::Texture {
                    texture: "default://assets/block/grass_top.png".to_string(),
                    side_texture: Some("default://assets/block/grass_block_side.png".to_string()),
                    bottom_texture: Some("default://assets/block/dirt.png".to_string()),
                },
            ),
        );
        m.insert(
            2,
            BlockType::new(
                VoxelVisibility::Opaque,
                BlockContent::Texture {
                    texture: "default://assets/block/stone.png".to_string(),
                    side_texture: None,
                    bottom_texture: None,
                },
            ),
        );
        m
    };
}
