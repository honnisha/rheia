use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::voxel_visibility::VoxelVisibility;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BlockContent {
    Texture {
        texture: String,
        side_texture: Option<String>,
        bottom_texture: Option<String>,
    },
    ModelCube {
        model: String,
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, EnumIter)]
#[serde(rename_all = "snake_case")]
pub enum BlockCategory {
    Base,
    Furniture,
}

impl BlockCategory {
    pub fn external_iter() -> BlockCategoryIter {
        BlockCategory::iter()
    }

    pub fn to_str(&self) -> Cow<'static, str> {
        match *self {
            BlockCategory::Base => "base".into(),
            BlockCategory::Furniture => "furniture".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct BlockType {
    voxel_visibility: VoxelVisibility,
    block_content: BlockContent,
    category: BlockCategory,
}

impl BlockType {
    pub fn new(voxel_visibility: VoxelVisibility, block_content: BlockContent) -> Self {
        Self {
            voxel_visibility,
            block_content,
            category: BlockCategory::Base,
        }
    }

    pub fn category(mut self, category: BlockCategory) -> Self {
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
            BlockContent::ModelCube { model } => {
                return Some(model);
            }
            _ => None,
        }
    }
}
