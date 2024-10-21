use common::blocks::{
    block_type::{BlockContent, BlockType},
    voxel_visibility::VoxelVisibility,
};
use lazy_static::lazy_static;
use std::collections::HashMap;

pub const DEFAULT_MEDIA: &'static [&'static str] = &[
    "default/block/grass_top.png",
    "default/block/grass_block_side.png",
    "default/block/dirt.png",
    "default/block/stone.png",
];

lazy_static! {
    pub static ref DEFAULT_BLOCKS: HashMap<u32, BlockType> = {
        let mut m = HashMap::new();
        m.insert(
            1,
            BlockType::new(
                VoxelVisibility::Opaque,
                BlockContent::Texture {
                    texture: "default/block/grass_top.png".to_string(),
                    side_texture: Some("default/block/grass_block_side.png".to_string()),
                    bottom_texture: Some("default/block/dirt.png".to_string()),
                },
            ),
        );
        m.insert(
            2,
            BlockType::new(
                VoxelVisibility::Opaque,
                BlockContent::Texture {
                    texture: "default/block/stone.png".to_string(),
                    side_texture: None,
                    bottom_texture: None,
                },
            ),
        );
        m
    };
}
