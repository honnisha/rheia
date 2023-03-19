use crate::utils::block_mesh::{Voxel, VoxelVisibility};

#[derive(Copy)]
pub struct BlockInfo {
    id: i32,
}

impl BlockInfo {
    pub fn new(id: i32) -> BlockInfo {
        BlockInfo {
            id: id,
        }
    }

    pub fn is_solid(&self) -> bool {
        match self.id {
            0 => false,
            _ => true,
        }
    }
}

impl Clone for BlockInfo {
    fn clone(&self) -> BlockInfo {
        BlockInfo {
            id: self.id,
        }
    }
}

impl Voxel for BlockInfo {
    fn get_visibility(&self) -> VoxelVisibility {
        if self.is_solid() {
            VoxelVisibility::Empty
        } else {
            VoxelVisibility::Opaque
        }
    }
}
