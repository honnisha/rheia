use crate::{
    utils::mesh::block_mesh::VoxelVisibility,
    world::{
        blocks::blocks_storage::BlockType,
        chunks::chunk_info::{ChunkBordersShape, ChunkShape},
    },
};
use ndshape::ConstShape;

use super::{
    chunk_info::{ChunkData, ChunkDataBordered, CHUNK_SIZE},
    chunks_manager::{ChunksInfoLockRead, WORLD_CHUNKS_FROM, WORLD_CHUNKS_TO},
};

pub fn format_chunk_data_with_boundaries(
    ci: &ChunksInfoLockRead,
    chunk_data: &ChunkData,
    chunk_pos: &[i32; 3],
) -> ChunkDataBordered {
    //use std::time::Instant;
    //let now = Instant::now();

    let mut b_chunk = [BlockType::Stone; ChunkBordersShape::SIZE as usize];

    let mut has_any_mesh = false;

    for x in 0_u32..(CHUNK_SIZE as u32) {
        for y in 0_u32..(CHUNK_SIZE as u32) {
            for z in 0_u32..(CHUNK_SIZE as u32) {
                let i = ChunkShape::linearize([x, y, z]);
                assert!(
                    i < ChunkShape::SIZE,
                    "Generate chunk data overflow array length; dimentions:{:?} current:{:?}",
                    ChunkShape::ARRAY,
                    [x, y, z]
                );

                let b_chunk_pos = ChunkBordersShape::linearize([x + 1, y + 1, z + 1]);
                let data = chunk_data[i as usize];
                b_chunk[b_chunk_pos as usize] = data.get_block_type().clone();

                if *data
                    .get_block_type()
                    .get_block_type_info()
                    .unwrap()
                    .get_voxel_visibility()
                    != VoxelVisibility::Empty
                {
                    has_any_mesh = true;
                }
            }
        }
    }

    // fill boundaries
    if !has_any_mesh {
        return b_chunk;
    }
    //godot_print!("chunk:{:?}", chunk_pos);

    let boundary = get_boundaries_chunks(&chunk_pos);
    for (axis, value, pos) in boundary {
        //godot_print!("load:{:?}", pos);

        if pos[1] >= WORLD_CHUNKS_TO || pos[1] < WORLD_CHUNKS_FROM {
            continue;
        }
        let border_chunk_info = match ci.get(&pos) {
            Some(c) => c,
            _ => {
                println!("FORMAT_CHUNK_DATA_WITH_BOUNDARIES chunk {:?} must be loaded", pos);
                return b_chunk;
            }
        };

        for i in 0_u32..(CHUNK_SIZE as u32) {
            for j in 0_u32..(CHUNK_SIZE as u32) {
                let (i_v, o_v) = match value {
                    -1 => (0, CHUNK_SIZE as u32 - 1),
                    _ => (CHUNK_SIZE as u32 + 1, 0),
                };

                let (pos_inside, pos_outside) = match axis {
                    0 => ([i_v, i + 1, j + 1], [o_v, i, j]),
                    1 => ([i + 1, i_v, j + 1], [i, o_v, j]),
                    _ => ([i + 1, j + 1, i_v], [i, j, o_v]),
                };

                let pos_i = ChunkBordersShape::linearize(pos_inside);
                let pos_o = ChunkShape::linearize(pos_outside);
                //godot_print!(
                //    "pos_inside:{:?} pos_outside:{:?}",
                //    pos_inside,
                //    pos_outside
                //);
                b_chunk[pos_i as usize] = border_chunk_info.get_chunk_data()[pos_o as usize]
                    .get_block_type()
                    .clone();
            }
        }
    }

    //println!(
    //    "format_chunk_data_with_boundaries {:?} data generated in {:.2?}",
    //    chunk_pos,
    //    now.elapsed()
    //);
    return b_chunk;
}

/// (axis, value, chunk_pos)
pub fn get_boundaries_chunks(chunk_pos: &[i32; 3]) -> Vec<(i8, i32, [i32; 3])> {
    let mut result = Vec::with_capacity(6);

    for axis in 0_i8..3_i8 {
        for value in (-1_i32..2_i32).step_by(2) {
            let mut pos = chunk_pos.clone();

            pos[axis as usize] += value;
            result.push((axis, value, pos));
        }
    }
    result
}
