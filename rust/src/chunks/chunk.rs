use godot::prelude::Vector3;
use ndshape::ConstShape;

use crate::mesh::mesh_generator::ChunkShape;

use super::block_info::BlockInfo;

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

    pub fn get_position(&self) -> [i32; 3] {
        self.position
    }

    #[allow(dead_code)]
    pub fn get_block_info(&self, position: [u32; 3]) -> BlockInfo {
        return self.chunk_data[ChunkShape::linearize(position) as usize];
    }

    pub fn get_chunk_position(&self) -> Vector3 {
        Chunk::get_chunk_position_from_position(&self.position)
    }

    pub fn get_chunk_position_from_position(position: &[i32; 3]) -> Vector3 {
        Vector3::new(
            position[0] as f32 * 16.0 - 8.0,
            position[1] as f32 * 16.0 - 8.0,
            position[2] as f32 * 16.0 - 8.0,
        )
    }
}
