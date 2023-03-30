use crate::utils::mesh::block_mesh::VoxelVisibility;

pub struct BlockTypeInfo {
    pub voxel_visibility: VoxelVisibility,

    pub top_texture: Option<&'static str>,
    pub side_texture: Option<&'static str>,
    pub bottom_texture: Option<&'static str>,
}

impl BlockTypeInfo {
    pub const fn new_empty() -> Self {
        BlockTypeInfo {
            voxel_visibility: VoxelVisibility::Empty,
            top_texture: None,
            side_texture: None,
            bottom_texture: None,
        }
    }
    pub const fn new_opaque_mono_side(texture: &'static str) -> Self {
        BlockTypeInfo {
            voxel_visibility: VoxelVisibility::Opaque,
            top_texture: Some(texture),
            side_texture: Some(texture),
            bottom_texture: Some(texture),
        }
    }
    pub const fn new_opaque_mono_translucent(texture: &'static str) -> Self {
        BlockTypeInfo {
            voxel_visibility: VoxelVisibility::Translucent,
            top_texture: Some(texture),
            side_texture: Some(texture),
            bottom_texture: Some(texture),
        }
    }

    pub fn get_voxel_visibility(&self) -> &VoxelVisibility {
        &self.voxel_visibility
    }
}
