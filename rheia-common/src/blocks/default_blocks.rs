use crate::blocks::{
    block_type::{BlockContent, BlockType},
    voxel_visibility::VoxelVisibility,
};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DEFAULT_BLOCKS: Vec<BlockType> = {
        let mut m = Vec::new();
        m.push(BlockType::new(
            "grass",
            VoxelVisibility::Opaque,
            BlockContent::Texture {
                texture: "default://assets/block/grass_top.png".to_string(),
                side_texture: Some("default://assets/block/grass_block_side.png".to_string()),
                bottom_texture: Some("default://assets/block/dirt.png".to_string()),
            },
        ));
        m.push(BlockType::new(
            "stone",
            VoxelVisibility::Opaque,
            BlockContent::single_texture("default://assets/block/stone.png"),
        ));
        m.push(BlockType::new(
            "sand",
            VoxelVisibility::Opaque,
            BlockContent::single_texture("default://assets/block/sand.png"),
        ));
        m.push(BlockType::new(
            "water",
            VoxelVisibility::Translucent,
            BlockContent::single_texture("default://assets/block/water_still.png"),
        ));
        m.push(BlockType::new(
            "acacia_log",
            VoxelVisibility::Opaque,
            BlockContent::texture(
                "default://assets/block/acacia_log_top.png",
                Some("default://assets/block/acacia_log.png"),
                None,
            ),
        ));
        m.push(BlockType::new(
            "acacia_planks",
            VoxelVisibility::Translucent,
            BlockContent::single_texture("default://assets/block/acacia_planks.png"),
        ));
        m.push(BlockType::new(
            "amethyst_block",
            VoxelVisibility::Translucent,
            BlockContent::single_texture("default://assets/block/amethyst_block.png"),
        ));
        m
    };
}
