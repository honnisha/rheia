use godot::engine::*;
use godot::obj::EngineEnum;
use godot::prelude::{Array, Gd, Vector3};
use godot::prelude::{PackedInt32Array, PackedVector2Array, PackedVector3Array, Variant};

use crate::mesh::mesh_generator::generate_buffer;
use crate::utils::block_mesh::RIGHT_HANDED_Y_UP_CONFIG;

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

    pub fn generate_geometry() -> Gd<ArrayMesh> {
        let mut arrays: Array<Variant> = Array::new();
        arrays.resize(mesh::ArrayType::ARRAY_MAX.ord() as usize);

        let buffer = generate_buffer();

        //let num_indices = buffer.num_quads() * 6;
        //let num_vertices = buffer.num_quads() * 4;
        //let mut _indices = Vec::with_capacity(num_indices);
        //let mut _positions = Vec::with_capacity(num_vertices);
        //let mut _normals = Vec::with_capacity(num_vertices);

        let mut indices = PackedInt32Array::new();
        let mut verts = PackedVector3Array::new();
        let mut normals = PackedVector3Array::new();
        let mut _uvs = PackedVector2Array::new();

        let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;
        for (group, face) in buffer.groups.into_iter().zip(faces.into_iter()) {
            // face is OrientedBlockFace
            for quad in group.into_iter() {
                for i in &face.quad_mesh_indices(verts.len() as u32) {
                    indices.push(i.to_owned() as i32);
                }

                for i in &face.quad_mesh_positions(&quad.into(), 1.0) {
                    verts.push(Vector3::new(i[0], i[1], i[2]));
                }

                for i in &face.quad_mesh_normals() {
                    normals.push(Vector3::new(i[0], i[1], i[2]));
                }

                //_indices.extend_from_slice(&face.quad_mesh_indices(_positions.len() as u32));
                //_positions.extend_from_slice(&face.quad_mesh_positions(&quad.into(), 1.0));
                //_normals.extend_from_slice(&face.quad_mesh_normals());
            }
        }

        arrays.set(
            mesh::ArrayType::ARRAY_INDEX.ord() as usize,
            Variant::from(indices),
        );
        arrays.set(
            mesh::ArrayType::ARRAY_VERTEX.ord() as usize,
            Variant::from(verts),
        );
        arrays.set(
            mesh::ArrayType::ARRAY_NORMAL.ord() as usize,
            Variant::from(normals),
        );
        //arrays.set(
        //    mesh::ArrayType::ARRAY_TEX_UV.ord() as usize,
        //    Variant::from(uvs),
        //);

        // godot_print!("ARRAY_MAX:{} arrays.len():{}", mesh::ArrayType::ARRAY_MAX.ord(), arrays.len());

        let mut mesh_ist = ArrayMesh::new();
        mesh_ist.add_surface_from_arrays(
            mesh::PrimitiveType::PRIMITIVE_TRIANGLES,
            arrays,
            Default::default(),
            Default::default(),
            mesh::ArrayFormat::ARRAY_FORMAT_VERTEX,
        );
        mesh_ist
    }

    pub fn get_mesh(&mut self) -> Gd<MeshInstance3D> {
        let mut mesh_instance = MeshInstance3D::new_alloc();

        mesh_instance.set_mesh(Chunk::generate_geometry().upcast());
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
