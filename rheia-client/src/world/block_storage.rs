use common::blocks::block_info::BlockIndexType;
use common::blocks::block_type::{BlockContent, BlockType};
use common::blocks::default_blocks::DEFAULT_BLOCKS;
use std::collections::{BTreeMap, HashMap};

use crate::client_scripts::resource_manager::ResourceStorage;

pub struct BlockStorage {
    blocks: BTreeMap<BlockIndexType, BlockType>,
}

impl Default for BlockStorage {
    fn default() -> Self {
        let mut block_storage = Self {
            blocks: Default::default(),
        };
        for (i, block_type) in DEFAULT_BLOCKS.iter() {
            block_storage.blocks.insert(i.clone(), block_type.clone());
        }
        block_storage
    }
}

impl BlockStorage {
    pub fn get(&self, k: &BlockIndexType) -> Option<&BlockType> {
        self.blocks.get(k)
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, BlockIndexType, BlockType> {
        self.blocks.iter()
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
        block_types: HashMap<BlockIndexType, BlockType>,
        resources_storage: &ResourceStorage,
    ) -> Result<(), String> {
        self.blocks.clear();
        for (i, block_type) in block_types.iter() {
            match block_type.get_block_content() {
                BlockContent::Texture {
                    texture,
                    side_texture,
                    bottom_texture,
                } => {
                    if !resources_storage.has_media(texture) {
                        return Err(format!("texture not found: &e\"{}\"", texture));
                    }
                    if side_texture.is_some() && !resources_storage.has_media(&side_texture.as_ref().unwrap()) {
                        return Err(format!("texture not found: &e\"{}\"", side_texture.as_ref().unwrap()));
                    }
                    if bottom_texture.is_some() && !resources_storage.has_media(&bottom_texture.as_ref().unwrap()) {
                        return Err(format!("texture not found: &e\"{}\"", bottom_texture.as_ref().unwrap()));
                    }
                }
                BlockContent::ModelCube { model, icon_size: _ } => {
                    if !resources_storage.has_media(model) {
                        return Err(format!("model not found: &e\"{}\"", model));
                    }
                }
            }
            self.blocks.insert(i.clone(), block_type.clone());
        }
        return Ok(());
    }
}
