use std::{collections::HashMap, sync::{Arc, RwLock}};

use ndshape::ConstShape;

use crate::{utils::mesh::{mesh_generator::{ChunkShape, ChunkBordersShape}, block_mesh::VoxelVisibility}, world::{blocks::blocks_storage::BlockType, world_generator::WorldGenerator}};

use super::{block_info::BlockInfo, chunk_info::ChunkInfo, chunks_manager::ChunksManager};

pub fn format_chunk_data_with_boundaries(
    world_generator: Arc<RwLock<WorldGenerator>>,
    chunks_info: &mut HashMap<[i32; 3], ChunkInfo>,
    chunk_data: &[BlockInfo; 4096],
    chunk_position: &[i32; 3],
) -> [BlockType; 5832] {
    let mut b_chunk = [BlockType::Air; 5832];

    let mut has_any_mesh = false;

    for x in 0_u32..16_u32 {
        for y in 0_u32..16_u32 {
            for z in 0_u32..16_u32 {
                let i = ChunkShape::linearize([x, y, z]);
                assert!(
                    i < ChunkShape::SIZE,
                    "Generate chunk data overflow array length; dimentions:{:?} current:{:?}",
                    ChunkShape::ARRAY,
                    [x, y, z]
                );

                let b_chunk_position = ChunkBordersShape::linearize([x + 1, y + 1, z + 1]);
                let data = chunk_data[i as usize];
                b_chunk[b_chunk_position as usize] = data.get_block_type().clone();

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
    if has_any_mesh {
        //godot_print!("chunk:{:?}", chunk_position);

        for axis in 0_i8..3_i8 {
            for value in (-1_i32..2_i32).step_by(2) {
                let mut pos = chunk_position.clone();

                pos[axis as usize] += value;
                //godot_print!("load:{:?}", pos);

                let border_chunk_info = ChunksManager::get_or_load_chunk_data(
                    &world_generator,
                    chunks_info,
                    &pos,
                );

                for i in 0_u32..16_u32 {
                    for j in 0_u32..16_u32 {
                        let (i_v, o_v) = match value {
                            -1 => (0, 15),
                            _ => (17, 0),
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
        }
    }

    return b_chunk;
}
