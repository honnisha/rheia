use arrayvec::ArrayVec;
use common::{
    blocks::{blocks_storage::BlockType, voxel_visibility::VoxelVisibility},
    chunks::block_position::ChunkBlockPosition,
    network::messages::ChunkDataType,
    CHUNK_SIZE, VERTICAL_SECTIONS,
};
use ndshape::ConstShape;

use super::{
    chunk_column::ColumnDataLockType,
    chunk_section::{ChunkBordersShape, ChunkDataBordered},
    near_chunk_data::NearChunksData,
};

pub fn format_chunk_data_with_boundaries(
    chunks_near: Option<&NearChunksData>,
    chunk_data: &ColumnDataLockType,
    y: usize,
) -> ChunkDataBordered {
    // Fill with solid block by default
    let mut b_chunk = [BlockType::Stone; ChunkBordersShape::SIZE as usize];

    let mut mesh_count = 0;

    let cd = chunk_data.read();
    let section_data = cd.get(y).unwrap();

    for x in 0_u32..(CHUNK_SIZE as u32) {
        for y in 0_u32..(CHUNK_SIZE as u32) {
            for z in 0_u32..(CHUNK_SIZE as u32) {
                let b_chunk_pos = ChunkBordersShape::linearize([x + 1, y + 1, z + 1]);
                let block_type = match section_data.get(&ChunkBlockPosition::new(x as u8, y as u8, z as u8)) {
                    Some(e) => e.get_block_type(),
                    None => BlockType::Air,
                };

                if block_type.get_block_type_info().unwrap().get_voxel_visibility() != VoxelVisibility::Empty {
                    mesh_count += 1
                }

                b_chunk[b_chunk_pos as usize] = block_type;
            }
        }
    }

    // println!("mesh_count: {}", mesh_count);

    // fill boundaries
    if mesh_count == 0 {
        return b_chunk;
    }

    let chunks_near = match chunks_near {
        Some(c) => c,
        None => {
            return b_chunk;
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
                let block_type = match chunk_section.as_ref() {
                    Some(c) => match c.get(&pos_o) {
                        Some(e) => e.get_block_type(),
                        None => BlockType::Air,
                    },
                    None => BlockType::Air,
                };
                b_chunk[pos_i as usize] = block_type;
            }
        }
    }

    return b_chunk;
}

type BondaryType<'a> = ArrayVec<(i8, i32, Option<Box<ChunkDataType>>), 6>;

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

fn get_section<'a>(column: &'a Option<ColumnDataLockType>, y: usize) -> Option<Box<ChunkDataType>> {
    match column {
        Some(c) => match c.read().get(y) {
            Some(r) => Some(r.clone()),
            None => None,
        },
        None => None,
    }
}
