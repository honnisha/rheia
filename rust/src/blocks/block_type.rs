use crate::utils::block_mesh::{Voxel, VoxelVisibility};

use super::{
    block_type_info::BlockTypeInfo,
    blocks_storage::{get_block_type_info, BlockType},
};

impl BlockType {
    pub fn get_block_type_info(&self) -> Option<&'static BlockTypeInfo> {
        get_block_type_info(self)
    }
}

impl Voxel for BlockType {
    fn get_visibility(&self) -> VoxelVisibility {
        match get_block_type_info(self) {
            Some(t) => t.voxel_visibility.clone(),
            None => VoxelVisibility::Empty,
        }
    }
    fn get_type(&self) -> &BlockType {
        return self;
    }
}
