use strum_macros::Display;

use super::blocks_storage::BlockType;

/// Describes how this voxel influences mesh generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum VoxelVisibility {
    /// This voxel should not produce any geometry.
    Empty,
    /// Should produce geometry, and also light can pass through.
    Translucent,
    /// Light cannot pass through this voxel.
    Opaque,
}

/// Implement on your voxel types to inform the library
/// how to generate geometry for this voxel.
pub trait Voxel {
    fn get_visibility(&self) -> VoxelVisibility;
    fn get_type(&self) -> &BlockType;
}

/// Used as a dummy for functions that must wrap a voxel
/// but don't want to change the original's properties.
pub struct IdentityVoxel<'a, T: Voxel>(&'a T);

impl<'a, T: Voxel> Voxel for IdentityVoxel<'a, T> {
    #[inline]
    fn get_visibility(&self) -> VoxelVisibility {
        self.0.get_visibility()
    }
    fn get_type(&self) -> &BlockType {
        self.0.get_type()
    }
}

impl<'a, T: Voxel> From<&'a T> for IdentityVoxel<'a, T> {
    fn from(voxel: &'a T) -> Self {
        Self(voxel)
    }
}
