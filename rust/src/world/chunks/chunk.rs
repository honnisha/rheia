use godot::prelude::Vector3;
use ndshape::ConstShape;

use crate::{utils::mesh::mesh_generator::ChunkShape, world::blocks::blocks_storage::BlockType};

use super::block_info::BlockInfo;

#[allow(dead_code)]
pub struct Chunk {
    position: [i32; 3],
    pub chunk_data: [BlockInfo; 4096],
    pub loaded: bool,
}

impl Chunk {
    pub fn new(position: [i32; 3], chunk_data: [BlockInfo; 4096]) -> Self {
        Chunk {
            position: position,
            chunk_data: chunk_data,
            loaded: false,
        }
    }

    #[allow(dead_code)]
    pub fn get_position(&self) -> [i32; 3] {
        self.position
    }

    #[allow(dead_code)]
    pub fn get_block_info(&self, position: [u32; 3]) -> BlockInfo {
        return self.chunk_data[ChunkShape::linearize(position) as usize];
    }

    pub fn get_chunk_position_from_coordinate(position: &[i32; 3]) -> Vector3 {
        Vector3::new(
            position[0] as f32 * 16.0,
            position[1] as f32 * 16.0,
            position[2] as f32 * 16.0,
        )
    }

    pub fn get_chunk_positions_by_coordinate(c: &[i32; 3]) -> [i32; 3] {
        [c[0] % 16, c[1] % 16, c[2] % 16]
    }

    fn get_local_from_global(&self, global_pos: &[i32; 3]) -> [u32; 3] {
        [
            (global_pos[0] - (self.position[0] * 16_i32) as i32) as u32,
            (global_pos[1] - (self.position[1] * 16_i32) as i32) as u32,
            (global_pos[2] - (self.position[2] * 16_i32) as i32) as u32
        ]
    }

    pub fn set_block(&mut self, global_pos: &[i32; 3], block_type: BlockType) {
        let local_pos = self.get_local_from_global(global_pos);
        let i = ChunkShape::linearize(local_pos) as usize;
        self.chunk_data[i] = BlockInfo::new(block_type);
    }
}
