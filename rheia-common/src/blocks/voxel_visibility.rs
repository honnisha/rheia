use crate::chunks::chunk_data::BlockDataInfo;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

/// Describes how this voxel influences mesh generation.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Display)]
#[serde(rename_all = "snake_case")]
pub enum VoxelVisibility {
    /// This voxel should not produce any geometry.
    Empty,
    /// Should produce geometry, and also light can pass through.
    Translucent,
    /// Light cannot pass through this voxel.
    Opaque,
}

impl Default for VoxelVisibility {
    fn default() -> Self {
        Self::Opaque
    }
}

/// Implement on your voxel types to inform the library
/// how to generate geometry for this voxel.
pub trait Voxel {
    fn get_visibility(&self) -> VoxelVisibility;
    fn get_block_info(&self) -> &Option<BlockDataInfo>;
}

/// Used as a dummy for functions that must wrap a voxel
/// but don't want to change the original's properties.
pub struct IdentityVoxel<'a, T: Voxel>(&'a T);

impl<'a, T: Voxel> Voxel for IdentityVoxel<'a, T> {
    #[inline]
    fn get_visibility(&self) -> VoxelVisibility {
        self.0.get_visibility()
    }
    fn get_block_info(&self) -> &Option<BlockDataInfo> {
        self.0.get_block_info()
    }
}

impl<'a, T: Voxel> From<&'a T> for IdentityVoxel<'a, T> {
    fn from(voxel: &'a T) -> Self {
        Self(voxel)
    }
}
