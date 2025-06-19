use crate::blocks::block_type::{BlockType, BlockTypeManifest};
use lazy_static::lazy_static;

const DEFAULT_BLOCKS_YML: &str = r#"
- slug: grass
  block_content: !texture
    texture: default://assets/block/grass_block_top.png
    side_texture: default://assets/block/grass_block_side.png
    side_overlay: default://assets/block/grass_block_side_overlay.png
    bottom_texture: default://assets/block/dirt.png

- block_content: !texture
    texture: default://assets/block/stone.png
- block_content: !texture
    texture: default://assets/block/smooth_stone.png
- block_content: !texture
    texture: default://assets/block/stone_bricks.png
- block_content: !texture
    texture: default://assets/block/cracked_stone_bricks.png
- block_content: !texture
    texture: default://assets/block/mossy_stone_bricks.png

- block_content: !texture
    texture: res://assets/block/gravel.png
- block_content: !texture
    texture: res://assets/block/coarse_dirt.png
- block_content: !texture
    texture: res://assets/block/bedrock.png
- block_content: !texture
    texture: res://assets/block/sand.png
- block_content: !texture
    texture: res://assets/block/amethyst_block.png
- block_content: !texture
    texture: res://assets/block/bookshelf.png
- block_content: !texture
    texture: res://assets/block/iron_block.png

- block_content: !texture
    texture: default://assets/block/sandstone_top.png
    side_texture: default://assets/block/sandstone.png
    bottom_texture: default://assets/block/sandstone_bottom.png
- block_content: !texture
    texture: res://assets/block/chiseled_sandstone.png

- block_content: !texture
    texture: default://assets/block/podzol_top.png
    side_texture: default://assets/block/podzol_side.png

- block_content: !texture
    texture: res://assets/block/blackstone.png
- block_content: !texture
    texture: res://assets/block/polished_blackstone.png
- block_content: !texture
    texture: res://assets/block/andesite.png
- block_content: !texture
    texture: res://assets/block/deepslate.png
- block_content: !texture
    texture: res://assets/block/deepslate_bricks.png
- block_content: !texture
    texture: res://assets/block/cracked_deepslate_bricks.png
- block_content: !texture
    texture: res://assets/block/polished_deepslate.png
- block_content: !texture
    texture: res://assets/block/diorite.png
- block_content: !texture
    texture: res://assets/block/polished_diorite.png
- block_content: !texture
    texture: res://assets/block/granite.png
- block_content: !texture
    texture: res://assets/block/polished_granite.png
- block_content: !texture
    texture: res://assets/block/cobblestone.png
- block_content: !texture
    texture: res://assets/block/mossy_cobblestone.png

- block_content: !texture
    texture: res://assets/block/acacia_log_top.png
    side_texture: default://assets/block/acacia_log.png
  category: trees
- block_content: !texture
    texture: res://assets/block/acacia_leaves.png
  category: trees
- block_content: !texture
    texture: res://assets/block/acacia_planks.png
  category: trees
"#;

fn generate_default_blocks() -> Vec<BlockType> {
    let m: Vec<BlockTypeManifest> = serde_yaml::from_str(DEFAULT_BLOCKS_YML).unwrap();

    // println!("{}", serde_yaml::to_string(&m).unwrap());

    let m: Vec<BlockType> = m.iter().map(|m| m.to_block()).collect();
    m
}

lazy_static! {
    pub static ref DEFAULT_BLOCKS: Vec<BlockType> = generate_default_blocks();
}
