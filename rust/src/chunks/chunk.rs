use godot::engine::*;
use godot::prelude::{Gd, Vector3};
use ndshape::ConstShape;

use crate::mesh::mesh_generator::{generate_chunk_geometry, ChunkBordersShape, ChunkShape};

use super::block_info::BlockInfo;

pub struct Chunk {
    position: [i32; 3],
    pub mesh_instance: Gd<MeshInstance3D>,
    chunk_data: Option<[BlockInfo; 4096]>,
}

impl Chunk {
    pub fn new(position: [i32; 3]) -> Self {
        Chunk {
            position: position,
            mesh_instance: MeshInstance3D::new_alloc(),
            chunk_data: None,
        }
    }

    pub fn get_position(&self) -> [i32; 3] {
        self.position
    }

    pub fn set_chunk_data(&mut self, chunk_data: [BlockInfo; 4096]) {
        self.chunk_data = Some(chunk_data);
    }

    pub fn get_mesh(&mut self) -> Option<Gd<MeshInstance3D>> {
        let chunk_data = match self.chunk_data {
            Some(m) => m,
            None => return None,
        };
        let mut mesh_instance = MeshInstance3D::new_alloc();

        let chunk_data = self.format_chunk_data_with_boundaries(&chunk_data);

        let mesh = match generate_chunk_geometry(&chunk_data) {
            Some(m) => m,
            None => return None,
        };

        mesh_instance.set_mesh(mesh.upcast());
        mesh_instance.set_position(self.get_chunk_position());
        Some(mesh_instance)
    }

    #[allow(dead_code)]
    pub fn get_block_info(&self, position: [u32; 3]) -> Result<BlockInfo, String> {
        let chunk_data = match self.chunk_data {
            Some(m) => m,
            None => return Err("Chunk data is not initialized.".into()),
        };

        return Ok(chunk_data[ChunkShape::linearize(position) as usize]);
    }

    pub fn format_chunk_data_with_boundaries(
        &self,
        chunk_data: &[BlockInfo; 4096],
    ) -> [BlockInfo; 5832] {
        let mut chunk_boundaries_data = [BlockInfo::new(0); 5832];

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

                    let chunk_boundaries_position =
                        ChunkBordersShape::linearize([x + 1, y + 1, z + 1]);
                    chunk_boundaries_data[chunk_boundaries_position as usize] =
                        chunk_data[i as usize]
                }
            }
        }

        return chunk_boundaries_data;
    }

    pub fn get_chunk_position(&self) -> Vector3 {
        Vector3::new(
            self.position[0] as f32 * 16.0 - 8.0,
            self.position[1] as f32 * 16.0 - 8.0,
            self.position[2] as f32 * 16.0 - 8.0,
        )
    }
}
