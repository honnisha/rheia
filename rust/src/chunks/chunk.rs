use block_mesh::{OrientedBlockFace, RIGHT_HANDED_Y_UP_CONFIG};
use godot::engine::*;
use godot::obj::EngineEnum;
use godot::prelude::{Array, Gd, VariantArray, Vector3};
use godot::prelude::{PackedInt32Array, PackedVector2Array, PackedVector3Array, Variant};

use crate::mesh::mesh_generator::generate_buffer;

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

        // let num_indices = buffer.num_quads() * 6;
        // let num_vertices = buffer.num_quads() * 4;
        // let mut indices = Vec::with_capacity(num_indices);
        // let mut positions = Vec::with_capacity(num_vertices);
        // let mut normals = Vec::with_capacity(num_vertices);

        let mut verts = PackedVector3Array::new();
        let mut uvs = PackedVector2Array::new();
        let mut normals = PackedVector3Array::new();
        let mut indices = PackedInt32Array::new();

        let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;
        for (group, face) in buffer.groups.into_iter().zip(faces.into_iter()) {
            // face is OrientedBlockFace
            for quad in group.into_iter() {

                // indices.extend_from_slice(&face.quad_mesh_indices(positions.len() as u32));
                // positions.extend_from_slice(&face.quad_mesh_positions(&quad.into(), 1.0));
                // normals.extend_from_slice(&face.quad_mesh_normals());
            }
        }

        verts.push(Vector3::new(0.0, 1.0, 0.0));
        verts.push(Vector3::new(1.0, 0.0, 0.0));
        verts.push(Vector3::new(0.0, 0.0, 1.0));

        arrays.insert(
            mesh::ArrayType::ARRAY_VERTEX.ord() as usize,
            Variant::from(verts),
        );
        arrays.insert(
            mesh::ArrayType::ARRAY_TEX_UV.ord() as usize,
            Variant::from(uvs),
        );
        arrays.insert(
            mesh::ArrayType::ARRAY_NORMAL.ord() as usize,
            Variant::from(normals),
        );
        arrays.insert(
            mesh::ArrayType::ARRAY_INDEX.ord() as usize,
            Variant::from(indices),
        );

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
        let position = Vector3::new(
            self.position[0] as f32 * 16.0,
            self.position[1] as f32 * 16.0,
            self.position[2] as f32 * 16.0,
        );
        mesh_instance.set_position(position);
        mesh_instance
    }
}
