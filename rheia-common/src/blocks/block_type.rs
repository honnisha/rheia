use serde::{Deserialize, Serialize};

use super::voxel_visibility::VoxelVisibility;

const REGEX_FILE_NAME: &str = r"^(?:.*\/)*([a-zA-Z_0-9]+)(\.[a-zA-Z]+)";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ColliderType {
    Solid,
    Sensor,
}

impl ColliderType {
    pub fn is_sensor(&self) -> bool {
        match *self {
            ColliderType::Solid => false,
            ColliderType::Sensor => true,
        }
    }
}

impl Default for ColliderType {
    fn default() -> Self {
        Self::Solid
    }
}

pub type BlockColor = [u8; 3];

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BlockContent {
    Texture {
        texture: String,
        side_texture: Option<String>,
        side_overlay: Option<String>,
        bottom_texture: Option<String>,

        // Colors applied to texture and side_overlay
        colors_scheme: Option<Vec<BlockColor>>,
    },
    ModelCube {
        model: String,

        // #[serde(skip_serializing_if = "Option::is_none")]
        icon_size: Option<f32>,

        #[serde(default)]
        collider_type: ColliderType,
    },
}

impl BlockContent {
    pub fn is_texture(&self) -> bool {
        match self {
            BlockContent::Texture { .. } => true,
            _ => false,
        }
    }

    pub fn single<S: Into<String>>(texture: S) -> BlockContent {
        BlockContent::Texture {
            texture: texture.into(),
            side_texture: None,
            side_overlay: None,
            bottom_texture: None,
            colors_scheme: None,
        }
    }

    pub fn texture<S: Into<String>>(
        texture: S,
        side_texture: Option<S>,
        side_overlay: Option<S>,
        bottom_texture: Option<S>,
    ) -> BlockContent {
        BlockContent::Texture {
            texture: texture.into(),
            side_texture: match side_texture {
                Some(t) => Some(t.into()),
                None => None,
            },
            side_overlay: match side_overlay {
                Some(t) => Some(t.into()),
                None => None,
            },
            bottom_texture: match bottom_texture {
                Some(t) => Some(t.into()),
                None => None,
            },
            colors_scheme: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BlockTypeManifest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,

    #[serde(skip_serializing_if = "BlockType::is_default")]
    #[serde(default)]
    pub voxel_visibility: VoxelVisibility,

    pub block_content: BlockContent,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_color: Option<BlockColor>,
}

impl BlockTypeManifest {
    pub fn to_block(&self) -> BlockType {
        let category = match self.category.as_ref() {
            Some(c) => c.clone(),
            None => BlockType::default_category(),
        };
        let mut b = BlockType::new(self.block_content.clone())
            .category(category)
            .visibility(self.voxel_visibility.clone())
            .map_color(self.map_color.clone());
        if let Some(slug) = self.slug.as_ref() {
            b = b.set_slug(slug.clone());
        }
        b
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BlockType {
    slug: String,

    // For texturing and collider building
    voxel_visibility: VoxelVisibility,

    block_content: BlockContent,

    category: String,

    map_color: Option<BlockColor>,
}

impl BlockType {
    fn is_default<T: Default + PartialEq>(attr: &T) -> bool {
        *attr == T::default()
    }
    fn default_category() -> String {
        "base".to_string()
    }
}

impl BlockType {
    fn generate_slug(block_content: &BlockContent) -> String {
        let path = match &block_content {
            BlockContent::Texture { texture, .. } => texture.clone(),
            BlockContent::ModelCube { model, .. } => model.clone(),
        };
        let re = regex::Regex::new(REGEX_FILE_NAME).unwrap();
        let Some(re) = re.captures(&path) else {
            panic!("Path \"{}\" regex return None", path);
        };
        let Some(slug) = re.get(1) else {
            panic!("Path \"{}\" regex group not found", path);
        };
        slug.as_str().into()
    }

    pub fn new(block_content: BlockContent) -> Self {
        let slug = BlockType::generate_slug(&block_content);
        Self {
            slug: slug,
            voxel_visibility: VoxelVisibility::default(),
            block_content,
            category: BlockType::default_category(),
            map_color: None,
        }
    }

    pub fn map_color(mut self, map_color: Option<BlockColor>) -> Self {
        self.map_color = map_color;
        self
    }

    pub fn get_map_color(&self) -> Option<&BlockColor> {
        self.map_color.as_ref()
    }

    pub fn set_slug<S: Into<String>>(mut self, slug: S) -> Self {
        self.slug = slug.into();
        self
    }

    pub fn visibility(mut self, voxel_visibility: VoxelVisibility) -> Self {
        self.voxel_visibility = voxel_visibility;
        self
    }

    pub fn category(mut self, category: String) -> Self {
        self.category = category;
        self
    }

    pub fn get_slug(&self) -> &String {
        &self.slug
    }

    pub fn get_category(&self) -> &String {
        &self.category
    }

    pub fn get_voxel_visibility(&self) -> &VoxelVisibility {
        &self.voxel_visibility
    }

    pub fn get_block_content(&self) -> &BlockContent {
        &self.block_content
    }

    pub fn get_block_content_mut(&mut self) -> &mut BlockContent {
        &mut self.block_content
    }

    pub fn get_model(&self) -> Option<&String> {
        match &self.block_content {
            BlockContent::ModelCube { model, .. } => {
                return Some(model);
            }
            _ => None,
        }
    }
}
