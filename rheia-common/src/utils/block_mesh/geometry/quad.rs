use crate::chunks::chunk_data::BlockDataInfo;

/// The minimum voxel and size of a quad, without an orientation. To get the
/// actual corners of the quad, combine with an [`OrientedBlockFace`].
///
/// When using these values for materials and lighting, you can access them
/// using either the quad's minimum voxel coordinates or the vertex coordinates
/// given by `OrientedBlockFace::quad_corners`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnorientedQuad {
    /// The minimum voxel in the quad.
    pub minimum: [u32; 3],
    /// Width of the quad.
    pub width: u32,
    /// Height of the quad.
    pub height: u32,
}

impl From<UnorientedUnitQuad> for UnorientedQuad {
    #[inline]
    fn from(unit: UnorientedUnitQuad) -> Self {
        Self {
            minimum: unit.minimum,
            width: 1,
            height: 1,
        }
    }
}

/// A quad covering a single voxel (just a single block face), without an
/// orientation. To get the actual corners of the quad, combine with an
/// [`OrientedBlockFace`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnorientedUnitQuad {
    /// The minimum voxel in the quad.
    pub minimum: [u32; 3],
    pub block_info: Option<BlockDataInfo>,
}
