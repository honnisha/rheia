use crate::utils::block_mesh::greedy::MergeVoxel;

use super::{
    block_info::BlockInfo,
    voxel_visibility::{Voxel, VoxelVisibility},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkColliderInfo {
    voxel_visibility: VoxelVisibility,
    block_info: Option<BlockInfo>,
}

impl ChunkColliderInfo {
    pub fn create(voxel_visibility: VoxelVisibility, block_info: Option<BlockInfo>) -> Self {
        Self {
            voxel_visibility,
            block_info,
        }
    }
}

impl ChunkColliderInfo {
    pub fn get_voxel_visibility(&self) -> &VoxelVisibility {
        &self.voxel_visibility
    }
}

impl MergeVoxel for ChunkColliderInfo {
    type MergeValue = Self;
    type MergeValueFacingNeighbour = Self;

    fn merge_value(&self) -> Self::MergeValue {
        self.clone()
    }

    fn merge_value_facing_neighbour(&self) -> Self::MergeValueFacingNeighbour {
        self.clone()
    }
}

impl Voxel for ChunkColliderInfo {
    fn get_visibility(&self) -> VoxelVisibility {
        self.voxel_visibility.clone()
    }

    fn get_block_info(&self) -> &Option<BlockInfo> {
        &self.block_info
    }
}
