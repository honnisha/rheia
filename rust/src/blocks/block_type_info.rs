use crate::utils::block_mesh::VoxelVisibility;

pub struct BlockTypeInfo {
    pub voxel_visibility: VoxelVisibility,

    pub top_texture: Option<&'static str>,
    pub side_texture: Option<&'static str>,
    pub bottom_texture: Option<&'static str>,
}

impl BlockTypeInfo {
    pub fn get_voxel_visibility(&self) -> &VoxelVisibility {
        &self.voxel_visibility
    }
}
