use arrayvec::ArrayVec;
use common::{
    CHUNK_SIZE, VERTICAL_SECTIONS,
    blocks::{block_type::BlockType, chunk_collider_info::ChunkColliderInfo, voxel_visibility::VoxelVisibility},
    chunks::{
        block_position::ChunkBlockPosition,
        chunk_data::{BlockDataInfo, ChunkSectionData},
    },
};
use ndshape::ConstShape;

use crate::world::block_storage::BlockStorage;

use super::{
    chunk_column::ColumnDataLockType,
    chunk_section::{ChunkBordersShape, ChunkColliderDataBordered},
    near_chunk_data::NearChunksData,
};

fn get_collider(info: Option<&BlockDataInfo>, block_storage: &BlockStorage) -> Result<ChunkColliderInfo, String> {
    let collider = match info {
        Some(block_info) => {
            let block_type = match block_storage.get(&block_info.get_id()) {
                Some(b) => b,
                None => {
                    return Err(format!("Block type #{} not found", block_info.get_id()));
                }
            };
            if block_type.get_block_content().is_texture() {
                ChunkColliderInfo::create(block_type.get_voxel_visibility().clone(), Some(block_info.clone()))
            } else {
                ChunkColliderInfo::create(VoxelVisibility::Empty, None)
            }
        }
        None => ChunkColliderInfo::create(VoxelVisibility::Empty, None),
    };
    Ok(collider)
}

pub fn generate_single_block(block_type: &BlockType, block_info: &BlockDataInfo) -> ChunkColliderDataBordered {
    let mut b_chunk = [ChunkColliderInfo::create(VoxelVisibility::Empty, None); ChunkBordersShape::SIZE as usize];

    let collider = ChunkColliderInfo::create(block_type.get_voxel_visibility().clone(), Some(block_info.clone()));
    let b_chunk_pos = ChunkBordersShape::linearize([1, 1, 1]);
    b_chunk[b_chunk_pos as usize] = collider;
    b_chunk
}

/// Generates collider data for mesh
/// with size of CHUNK_SIZE + 2 boundary
pub fn format_chunk_data_with_boundaries(
    chunks_near: Option<&NearChunksData>,
    chunk_data: &ColumnDataLockType,
    block_storage: &BlockStorage,
    y: usize,
) -> Result<(ChunkColliderDataBordered, usize), String> {
    // Fill with solid block by default
    let mut b_chunk = [ChunkColliderInfo::create(VoxelVisibility::Opaque, None); ChunkBordersShape::SIZE as usize];

    let mut mesh_count = 0;

    let cd = chunk_data.read();
    let section_data = cd.get(y).unwrap();

    for x in 0_u32..(CHUNK_SIZE as u32) {
        for y in 0_u32..(CHUNK_SIZE as u32) {
            for z in 0_u32..(CHUNK_SIZE as u32) {
                let b_chunk_pos = ChunkBordersShape::linearize([x + 1, y + 1, z + 1]);
                let block_info = section_data.get(&ChunkBlockPosition::new(x as u8, y as u8, z as u8));

                let collider = match get_collider(block_info, block_storage) {
                    Ok(m) => m,
                    Err(e) => return Err(e),
                };

                if *collider.get_voxel_visibility() != VoxelVisibility::Empty {
                    mesh_count += 1
                }

                b_chunk[b_chunk_pos as usize] = collider;
            }
        }
    }

    // fill boundaries
    if mesh_count == 0 {
        return Ok((b_chunk, mesh_count));
    }

    let chunks_near = match chunks_near {
        Some(c) => c,
        None => {
            return Ok((b_chunk, mesh_count));
        }
    };
    let boundary = get_boundaries_chunks(&chunks_near, &chunk_data, y);
    for (axis, axis_diff, chunk_section) in boundary {
        for i in 0_u32..(CHUNK_SIZE as u32) {
            for j in 0_u32..(CHUNK_SIZE as u32) {
                let (i_v, o_v) = match axis_diff {
                    -1 => (0, CHUNK_SIZE as u32 - 1),
                    _ => (CHUNK_SIZE as u32 + 1, 0),
                };

                let (pos_inside, pos_outside) = match axis {
                    0 => ([i_v, i + 1, j + 1], [o_v, i, j]),
                    1 => ([i + 1, i_v, j + 1], [i, o_v, j]),
                    _ => ([i + 1, j + 1, i_v], [i, j, o_v]),
                };

                let pos_i = ChunkBordersShape::linearize(pos_inside);

                let pos_o = ChunkBlockPosition::new(pos_outside[0] as u8, pos_outside[1] as u8, pos_outside[2] as u8);

                let collider = match chunk_section.as_ref() {
                    Some(border_chunk_data) => {
                        let block_info = border_chunk_data.get(&pos_o);
                        match get_collider(block_info, block_storage) {
                            Ok(m) => m,
                            Err(e) => return Err(e),
                        }
                    }
                    None => ChunkColliderInfo::create(VoxelVisibility::Empty, None),
                };
                b_chunk[pos_i as usize] = collider;
            }
        }
    }

    return Ok((b_chunk, mesh_count));
}

type BondaryType<'a> = ArrayVec<(i8, i32, Option<Box<ChunkSectionData>>), 6>;

fn get_boundaries_chunks<'a>(
    chunks_near: &'a NearChunksData,
    chunk_data: &'a ColumnDataLockType,
    y: usize,
) -> BondaryType<'a> {
    let mut result: BondaryType = Default::default();

    let current_section = chunk_data.read();
    // x, y, z
    for axis in 0_i8..3_i8 {
        for axis_diff in (-1_i32..2_i32).step_by(2) {
            let side = match axis {
                // x
                0 => {
                    if axis_diff == -1 {
                        get_section(&chunks_near.forward, y)
                    } else {
                        get_section(&chunks_near.behind, y)
                    }
                }

                // y
                1 => {
                    let index = y as i32 + axis_diff;
                    if index >= 0 && index < VERTICAL_SECTIONS as i32 {
                        match current_section.get(index as usize) {
                            Some(r) => Some(r.clone()),
                            None => None,
                        }
                    } else {
                        None
                    }
                }

                // z
                2 => {
                    if axis_diff == -1 {
                        get_section(&chunks_near.left, y)
                    } else {
                        get_section(&chunks_near.right, y)
                    }
                }
                _ => panic!(),
            };

            result.push((axis, axis_diff, side));
        }
    }
    result
}

fn get_section<'a>(column: &'a Option<ColumnDataLockType>, y: usize) -> Option<Box<ChunkSectionData>> {
    match column {
        Some(c) => match c.read().get(y) {
            Some(r) => Some(r.clone()),
            None => None,
        },
        None => None,
    }
}
