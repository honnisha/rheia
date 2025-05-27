use super::voxel_visibility::{Voxel, VoxelVisibility};
use crate::{chunks::chunk_data::BlockDataInfo, utils::block_mesh::greedy::MergeVoxel};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkColliderInfo {
    voxel_visibility: VoxelVisibility,
    block_info: Option<BlockDataInfo>,
}

impl ChunkColliderInfo {
    pub fn create(voxel_visibility: VoxelVisibility, block_info: Option<BlockDataInfo>) -> Self {
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

    fn get_block_info(&self) -> &Option<BlockDataInfo> {
        &self.block_info
    }
}
