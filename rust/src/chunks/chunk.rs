use godot::engine::*;
use godot::prelude::{Gd, Vector3};

use crate::mesh::mesh_generator::generate_chunk_geometry;

pub struct Chunk {
    position: [i32; 3],
    pub mesh_instance: Gd<MeshInstance3D>,
}

impl Chunk {
    pub fn new(position: [i32; 3]) -> Self {
        Chunk {
            position: position,
            mesh_instance: MeshInstance3D::new_alloc(),
        }
    }

    pub fn get_position(&self) -> [i32; 3] {
        self.position
    }

    pub fn get_mesh(&mut self) -> Gd<MeshInstance3D> {
        let mut mesh_instance = MeshInstance3D::new_alloc();

        mesh_instance.set_mesh(generate_chunk_geometry().upcast());
        mesh_instance.set_position(self.get_chunk_position());
        mesh_instance
    }

    pub fn get_chunk_position(&self) -> Vector3 {
        Vector3::new(
            self.position[0] as f32 * 16.0 - 8.0,
            self.position[1] as f32 * 16.0 - 8.0,
            self.position[2] as f32 * 16.0 - 8.0,
        )
    }
}
