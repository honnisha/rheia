use crate::blocks::block_type::{BlockType, BlockTypeManifest};

/// Их необходимо хранить в общей библиотеки, т.к. их используют клиент и сервер
/// Клиент загружает их по дефолту
/// Сервер сохраняет id в соответствии со slug блоков и передает на клиент
const DEFAULT_BLOCKS_YML: &str = r#"
- slug: grass
  block_content: !texture
    texture: default://assets/block/grass_block_top.png
    side_texture: default://assets/block/grass_block_side.png
    side_overlay: default://assets/block/grass_block_side_overlay.png
    bottom_texture: default://assets/block/dirt.png
    colors_scheme:
      - [76, 187, 23]
  map_color: [76, 187, 23]

- slug: water
  block_content: !texture
    texture: default://assets/block/water_overlay.png
  voxel_visibility: translucent

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
    texture: default://assets/block/gravel.png
- block_content: !texture
    texture: default://assets/block/coarse_dirt.png
- block_content: !texture
    texture: default://assets/block/bedrock.png
- block_content: !texture
    texture: default://assets/block/sand.png
- block_content: !texture
    texture: default://assets/block/amethyst_block.png
- block_content: !texture
    texture: default://assets/block/oak_planks.png
    side_texture: default://assets/block/bookshelf.png
- block_content: !texture
    texture: default://assets/block/iron_block.png

- slug: sandstone
  block_content: !texture
    texture: default://assets/block/sandstone_top.png
    side_texture: default://assets/block/sandstone.png
    bottom_texture: default://assets/block/sandstone_bottom.png
- block_content: !texture
    texture: default://assets/block/chiseled_sandstone.png

- slug: podzol
  block_content: !texture
    texture: default://assets/block/podzol_top.png
    side_texture: default://assets/block/podzol_side.png

- block_content: !texture
    texture: default://assets/block/blackstone.png
- block_content: !texture
    texture: default://assets/block/polished_blackstone.png
- block_content: !texture
    texture: default://assets/block/andesite.png
- block_content: !texture
    texture: default://assets/block/deepslate.png
- block_content: !texture
    texture: default://assets/block/deepslate_bricks.png
- block_content: !texture
    texture: default://assets/block/cracked_deepslate_bricks.png
- block_content: !texture
    texture: default://assets/block/polished_deepslate.png
- block_content: !texture
    texture: default://assets/block/diorite.png
- block_content: !texture
    texture: default://assets/block/polished_diorite.png
- block_content: !texture
    texture: default://assets/block/granite.png
- block_content: !texture
    texture: default://assets/block/polished_granite.png
- block_content: !texture
    texture: default://assets/block/cobblestone.png
- block_content: !texture
    texture: default://assets/block/mossy_cobblestone.png

- slug: acacia_log
  block_content: !texture
    texture: default://assets/block/acacia_log_top.png
    side_texture: default://assets/block/acacia_log.png
  category: trees
- block_content: !texture
    texture: default://assets/block/acacia_leaves.png
  category: trees
- block_content: !texture
    texture: default://assets/block/acacia_planks.png
  category: trees

- slug: birch_log
  block_content: !texture
    texture: default://assets/block/birch_log_top.png
    side_texture: default://assets/block/birch_log.png
  category: trees
- block_content: !texture
    texture: default://assets/block/birch_leaves.png
  category: trees
- block_content: !texture
    texture: default://assets/block/birch_planks.png
  category: trees

- slug: dark_oak
  block_content: !texture
    texture: default://assets/block/dark_oak_log_top.png
    side_texture: default://assets/block/dark_oak_log.png
  category: trees
- block_content: !texture
    texture: default://assets/block/dark_oak_leaves.png
  category: trees
- block_content: !texture
    texture: default://assets/block/dark_oak_planks.png
  category: trees

- slug: jungle_log
  block_content: !texture
    texture: default://assets/block/jungle_log_top.png
    side_texture: default://assets/block/jungle_log.png
  category: trees
- block_content: !texture
    texture: default://assets/block/jungle_leaves.png
  category: trees
- block_content: !texture
    texture: default://assets/block/jungle_planks.png
  category: trees

- slug: oak_log
  block_content: !texture
    texture: default://assets/block/oak_log_top.png
    side_texture: default://assets/block/oak_log.png
  category: trees
- block_content: !texture
    texture: default://assets/block/oak_leaves.png
  category: trees
- block_content: !texture
    texture: default://assets/block/oak_planks.png
  category: trees

- slug: spruce_log
  block_content: !texture
    texture: default://assets/block/spruce_log_top.png
    side_texture: default://assets/block/spruce_log.png
  category: trees
- block_content: !texture
    texture: default://assets/block/spruce_leaves.png
  category: trees
- block_content: !texture
    texture: default://assets/block/spruce_planks.png
  category: trees

-
  voxel_visibility: translucent
  block_content: !model_cube
    model: foliage://assets/resources/foliage/bush_small.glb
    collider_type: sensor
    icon_size: 1.4
  category: foliage
-
  voxel_visibility: translucent
  block_content: !model_cube
    model: foliage://assets/resources/foliage/flower_lupin.glb
    collider_type: sensor
    icon_size: 1.4
  category: foliage
-
  voxel_visibility: translucent
  block_content: !model_cube
    model: foliage://assets/resources/foliage/flower_lupin2.glb
    collider_type: sensor
    icon_size: 1.4
  category: foliage
-
  voxel_visibility: translucent
  block_content: !model_cube
    model: foliage://assets/resources/foliage/flower_orchid.glb
    collider_type: sensor
    icon_size: 1.4
  category: foliage
-
  voxel_visibility: translucent
  block_content: !model_cube
    model: foliage://assets/resources/foliage/flower_rose.glb
    collider_type: sensor
    icon_size: 1.4
  category: foliage
-
  voxel_visibility: translucent
  block_content: !model_cube
    model: foliage://assets/resources/foliage/flower_white.glb
    collider_type: sensor
    icon_size: 1.4
  category: foliage
-
  voxel_visibility: translucent
  block_content: !model_cube
    model: foliage://assets/resources/foliage/flower_white2.glb
    collider_type: sensor
    icon_size: 1.4
  category: foliage
-
  voxel_visibility: translucent
  block_content: !model_cube
    model: foliage://assets/resources/foliage/flower_yellow.glb
    collider_type: sensor
    icon_size: 1.4
  category: foliage
-
  voxel_visibility: translucent
  block_content: !model_cube
    model: foliage://assets/resources/foliage/flower_yellow2.glb
    collider_type: sensor
    icon_size: 1.4
  category: foliage
-
  voxel_visibility: translucent
  block_content: !model_cube
    model: foliage://assets/resources/foliage/flower_yellow3.glb
    collider_type: sensor
    icon_size: 1.4
  category: foliage
-
  voxel_visibility: translucent
  block_content: !model_cube
    model: foliage://assets/resources/foliage/grass1.glb
    collider_type: sensor
    icon_size: 1.4
  category: foliage
-
  voxel_visibility: translucent
  block_content: !model_cube
    model: foliage://assets/resources/foliage/grass2.glb
    collider_type: sensor
    icon_size: 1.4
  category: foliage
-
  voxel_visibility: translucent
  block_content: !model_cube
    model: foliage://assets/resources/foliage/grass3.glb
    collider_type: sensor
    icon_size: 1.4
  category: foliage
-
  voxel_visibility: translucent
  block_content: !model_cube
    model: foliage://assets/resources/foliage/grass4.glb
    collider_type: sensor
    icon_size: 1.4
  category: foliage
-
  voxel_visibility: translucent
  block_content: !model_cube
    model: foliage://assets/resources/foliage/ground_moss1.glb
    collider_type: sensor
    icon_size: 1.4
  category: foliage
-
  voxel_visibility: translucent
  block_content: !model_cube
    model: foliage://assets/resources/foliage/ground_moss2.glb
    collider_type: sensor
    icon_size: 1.4
  category: foliage
-
  voxel_visibility: translucent
  block_content: !model_cube
    model: foliage://assets/resources/foliage/ground_moss3.glb
    collider_type: sensor
    icon_size: 1.4
  category: foliage
-
  voxel_visibility: translucent
  block_content: !model_cube
    model: foliage://assets/resources/foliage/tall_grass1.glb
    collider_type: sensor
    icon_size: 1.4
  category: foliage
-
  voxel_visibility: translucent
  block_content: !model_cube
    model: foliage://assets/resources/foliage/tall_grass2.glb
    collider_type: sensor
    icon_size: 1.4
  category: foliage
"#;

pub fn generate_default_blocks() -> Result<Vec<BlockType>, String> {
    let m: Result<Vec<BlockTypeManifest>, serde_yaml::Error> = serde_yaml::from_str(DEFAULT_BLOCKS_YML);

    let m = match m {
        Ok(m) => m,
        Err(e) => return Err(format!("&cyaml parsing error: {}", e)),
    };
    // println!("{}", serde_yaml::to_string(&m).unwrap());

    let m: Vec<BlockType> = m.iter().map(|m| m.to_block()).collect();
    Ok(m)
}
