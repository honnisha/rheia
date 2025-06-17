use common::blocks::block_type::{BlockContent, BlockType};
use common::blocks::default_blocks::DEFAULT_BLOCKS;
use common::chunks::chunk_data::BlockIndexType;
use std::collections::BTreeMap;

use crate::client_scripts::resource_manager::ResourceStorage;

pub struct BlockStorage {
    blocks: BTreeMap<String, BlockType>,
    block_id_map: BTreeMap<BlockIndexType, String>,
}

impl Default for BlockStorage {
    fn default() -> Self {
        let mut block_storage = Self {
            blocks: Default::default(),
            block_id_map: Default::default(),
        };
        for block_type in DEFAULT_BLOCKS.iter() {
            block_storage
                .blocks
                .insert(block_type.get_slug().clone(), block_type.clone());
        }
        block_storage
    }
}

impl BlockStorage {
    pub fn get(&self, k: &BlockIndexType) -> Option<&BlockType> {
        let Some(slug) = self.block_id_map.get(k) else {
            panic!("BlockStorage can't find block id #{} in block_id_map", k);
        };
        self.blocks.get(slug)
    }

    pub fn set_block_id_map(&mut self, block_id_map: BTreeMap<BlockIndexType, String>) {
        self.block_id_map = block_id_map;
    }

    pub fn get_block_id(&self, slug: &String) -> Option<BlockIndexType> {
        for (block_id, block_slug) in self.block_id_map.iter() {
            if block_slug == slug {
                return Some(block_id.clone());
            }
        }
        None
    }

    pub fn iter_values(&self) -> std::collections::btree_map::Values<'_, String, BlockType> {
        self.blocks.values()
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, String, BlockType> {
        self.blocks.iter()
    }

    pub fn get_categories(&self) -> Vec<String> {
        let mut categories: Vec<String> = Vec::default();
        for (_block_id, block_type) in self.iter() {
            if !categories.contains(block_type.get_category()) {
                categories.push(block_type.get_category().clone());
            }
        }
        categories
    }
    pub fn textures_blocks_count(&self) -> i32 {
        let mut result = 0;
        for b in self.blocks.values() {
            if b.get_block_content().is_texture() {
                result += 1;
            }
        }
        result
    }

    /// Saves the server-side block scheme
    pub fn load_blocks_types(
        &mut self,
        block_types: Vec<BlockType>,
        resources_storage: &ResourceStorage,
    ) -> Result<(), String> {
        self.blocks.clear();
        for block_type in block_types.iter() {
            match block_type.get_block_content() {
                BlockContent::Texture {
                    texture,
                    side_texture,
                    bottom_texture,
                } => {
                    if !resources_storage.has_media(texture) {
                        return Err(format!(
                            "block \"{}\" &ctexture not found: \"{}\"",
                            block_type.get_slug(),
                            texture
                        ));
                    }
                    if side_texture.is_some() && !resources_storage.has_media(&side_texture.as_ref().unwrap()) {
                        return Err(format!(
                            "block \"{}\" &ctexture not found: \"{}\"",
                            block_type.get_slug(),
                            side_texture.as_ref().unwrap()
                        ));
                    }
                    if bottom_texture.is_some() && !resources_storage.has_media(&bottom_texture.as_ref().unwrap()) {
                        return Err(format!(
                            "block \"{}\" &ctexture not found: \"{}\"",
                            block_type.get_slug(),
                            bottom_texture.as_ref().unwrap()
                        ));
                    }
                }
                BlockContent::ModelCube { model, icon_size: _, collider_type: _ } => {
                    if !resources_storage.has_media(model) {
                        return Err(format!(
                            "block \"{}\" &cmodel not found: \"{}\"",
                            block_type.get_slug(),
                            model
                        ));
                    }
                }
            }
            self.blocks.insert(block_type.get_slug().clone(), block_type.clone());
        }
        return Ok(());
    }
}
