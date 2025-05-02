use serde::{Deserialize, Serialize};

use super::voxel_visibility::VoxelVisibility;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BlockContent {
    Texture {
        texture: String,
        side_texture: Option<String>,
        bottom_texture: Option<String>,
    },
    ModelCube {
        model: String,
        icon_size: Option<f32>,
    },
}

impl BlockContent {
    pub fn is_texture(&self) -> bool {
        match self {
            BlockContent::Texture {
                texture: _,
                side_texture: _,
                bottom_texture: _,
            } => true,
            _ => false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BlockType {
    voxel_visibility: VoxelVisibility,
    block_content: BlockContent,
    category: String,
}

impl BlockType {
    pub fn new(voxel_visibility: VoxelVisibility, block_content: BlockContent) -> Self {
        Self {
            voxel_visibility,
            block_content,
            category: "base".to_string(),
        }
    }

    pub fn get_category(&self) -> &String {
        &self.category
    }

    pub fn category(mut self, category: String) -> Self {
        self.category = category;
        self
    }

    pub fn get_voxel_visibility(&self) -> &VoxelVisibility {
        &self.voxel_visibility
    }

    pub fn get_block_content(&self) -> &BlockContent {
        &self.block_content
    }

    pub fn get_model(&self) -> Option<&String> {
        match &self.block_content {
            BlockContent::ModelCube { model, icon_size: _ } => {
                return Some(model);
            }
            _ => None,
        }
    }
}
