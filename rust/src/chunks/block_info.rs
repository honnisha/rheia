use crate::utils::block_mesh::{MergeVoxel, Voxel, VoxelVisibility};

use super::block_type::get_block_type_by_id;

#[derive(Copy)]
pub struct BlockInfo {
    type_id: i32,
}

impl BlockInfo {
    pub fn new(id: i32) -> BlockInfo {
        BlockInfo { type_id: id }
    }
}

impl Clone for BlockInfo {
    fn clone(&self) -> BlockInfo {
        BlockInfo {
            type_id: self.type_id,
        }
    }
}

impl Voxel for BlockInfo {
    fn get_visibility(&self) -> VoxelVisibility {
        get_block_type_by_id(self.type_id).voxel_visibility
    }
    fn get_id(&self) -> &i32 {
        return &self.type_id
    }
}

impl PartialEq for BlockInfo {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id
    }
}
impl Eq for BlockInfo {}
