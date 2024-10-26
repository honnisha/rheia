use common::blocks::block_type::{BlockContent, BlockType};
use std::collections::hash_map::Iter;
use std::collections::HashMap;

use crate::client_scripts::resource_manager::ResourceManager;

#[derive(Default)]
pub struct BlockStorage {
    blocks: HashMap<u32, BlockType>,
}

impl BlockStorage {
    pub fn get(&self, k: &u32) -> Option<&BlockType> {
        self.blocks.get(k)
    }

    pub fn iter(&self) -> Iter<'_, u32, BlockType> {
        self.blocks.iter()
    }

    /// Saves the server-side block scheme
    pub fn load_blocks_types(
        &mut self,
        block_types: HashMap<u32, BlockType>,
        resource_manager: &ResourceManager,
    ) -> Result<(), String> {
        self.blocks.clear();
        for (i, block_type) in block_types.iter() {
            match block_type.get_block_content() {
                BlockContent::Texture {
                    texture,
                    side_texture,
                    bottom_texture,
                } => {
                    if !resource_manager.has_media(texture) {
                        return Err(format!("texture not found: {}", texture));
                    }
                    if side_texture.is_some() && !resource_manager.has_media(&side_texture.as_ref().unwrap()) {
                        return Err(format!("texture not found: {}", side_texture.as_ref().unwrap()));
                    }
                    if bottom_texture.is_some() && !resource_manager.has_media(&bottom_texture.as_ref().unwrap()) {
                        return Err(format!("texture not found: {}", bottom_texture.as_ref().unwrap()));
                    }
                }
                BlockContent::ModelCube { model } => {
                    if !resource_manager.has_media(model) {
                        return Err(format!("model not found: {}", model));
                    }
                }
            }
            self.blocks.insert(i.clone(), block_type.clone());
        }
        return Ok(());
    }
}
