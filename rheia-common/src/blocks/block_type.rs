use super::voxel_visibility::VoxelVisibility;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockContent {
    Texture {
        texture: String,
        side_texture: Option<String>,
        bottom_texture: Option<String>,
    },
    ModelCube {
        voxel_visibility: VoxelVisibility,
        model: String,
    },
}

impl BlockContent {

    #[allow(unused_variables)]
    pub fn is_texture(&self) -> bool {
        match self {
            BlockContent::Texture { texture, side_texture, bottom_texture } => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockType {
    voxel_visibility: VoxelVisibility,
    block_content: BlockContent,
    selectable: bool,
}

impl BlockType {
    pub fn get_voxel_visibility(&self) -> &VoxelVisibility {
        &self.voxel_visibility
    }

    pub fn get_block_content(&self) -> &BlockContent {
        &self.block_content
    }
}
