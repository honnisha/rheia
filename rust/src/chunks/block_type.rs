use crate::utils::block_mesh::VoxelVisibility;

static BLOCKS_TYPE: &'static [BlockType] = &[
    // Air
    BlockType {
        voxel_visibility: VoxelVisibility::Empty,
        top_texture_offset: None,
        side_texture_offset: None,
        bottom_texture_offset: None,
    },
    // Grass
    BlockType {
        voxel_visibility: VoxelVisibility::Opaque,
        top_texture_offset: Some(1),
        side_texture_offset: Some(2),
        bottom_texture_offset: Some(3),
    },
    // Stone
    BlockType {
        voxel_visibility: VoxelVisibility::Opaque,
        top_texture_offset: Some(5),
        side_texture_offset: Some(5),
        bottom_texture_offset: Some(5),
    },
    // Water
    BlockType {
        voxel_visibility: VoxelVisibility::Translucent,
        top_texture_offset: Some(6),
        side_texture_offset: Some(6),
        bottom_texture_offset: Some(6),
    },
];

#[derive(Copy, Clone)]
pub struct BlockType {
    voxel_visibility: VoxelVisibility,

    pub top_texture_offset: Option<i16>,
    pub side_texture_offset: Option<i16>,
    pub bottom_texture_offset: Option<i16>,
}

impl BlockType {
    pub fn get_uv_offset(&self, side_index: i8) -> Option<i16> {
        match side_index {
            // Topside
            4 => self.top_texture_offset,
            // Bottom
            1 => self.bottom_texture_offset,
            // Sides
            _ => self.side_texture_offset,
        }
    }

    pub fn get_voxel_visibility(&self) -> &VoxelVisibility {
        &self.voxel_visibility
    }
}

pub fn get_block_type_by_id(id: &i32) -> BlockType {
    return BLOCKS_TYPE[*id as usize]
}
