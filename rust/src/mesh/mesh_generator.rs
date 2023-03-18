use crate::utils::block_mesh::{
    ilattice::prelude::Vec3A,
    ndshape::{ConstShape, ConstShape3u32},
    visible_block_faces, MergeVoxel, UnitQuadBuffer, Voxel, VoxelVisibility,
    RIGHT_HANDED_Y_UP_CONFIG, UnorientedQuad,
};
use godot::{obj::EngineEnum, prelude::{godot_print, Vector2}};
use godot::prelude::{Array, Gd, Vector3};
use godot::{engine::ArrayMesh, prelude::Variant};
use godot::{
    engine::*,
    prelude::{PackedInt32Array, PackedVector2Array, PackedVector3Array},
};

#[derive(Clone, Copy, Eq, PartialEq)]
struct BoolVoxel(bool);

const EMPTY: BoolVoxel = BoolVoxel(false);
// const FULL: BoolVoxel = BoolVoxel(true);

impl Voxel for BoolVoxel {
    fn get_visibility(&self) -> VoxelVisibility {
        if *self == EMPTY {
            VoxelVisibility::Empty
        } else {
            VoxelVisibility::Opaque
        }
    }
}

impl MergeVoxel for BoolVoxel {
    type MergeValue = Self;
    type MergeValueFacingNeighbour = Self;

    fn merge_value(&self) -> Self::MergeValue {
        *self
    }

    fn merge_value_facing_neighbour(&self) -> Self::MergeValueFacingNeighbour {
        *self
    }
}

// A 16^3 chunk with 1-voxel boundary padding.
type ChunkShape = ConstShape3u32<19, 19, 19>;

fn into_domain(array_dim: u32, [x, y, z]: [u32; 3]) -> Vec3A {
    (2.0 / array_dim as f32) * Vec3A::new(x as f32, y as f32, z as f32) - 1.0
}

pub fn generate_buffer() -> UnitQuadBuffer {
    let mut voxels = [EMPTY; ChunkShape::SIZE as usize];

    for i in 0u32..(ChunkShape::SIZE) {
        let pos = ChunkShape::delinearize(i);
        let p = into_domain(16, pos);
        voxels[i as usize] = BoolVoxel(pos[1] > 3_u32);
    }

    let mut buffer = UnitQuadBuffer::new();
    visible_block_faces(
        &voxels,
        &ChunkShape {},
        [0; 3],
        [17; 3],
        &RIGHT_HANDED_Y_UP_CONFIG.faces,
        &mut buffer,
    );
    buffer
}

pub fn generate_chunk_geometry() -> Gd<ArrayMesh> {
    let mut arrays: Array<Variant> = Array::new();
    arrays.resize(mesh::ArrayType::ARRAY_MAX.ord() as usize);

    let buffer = generate_buffer();

    let mut indices = PackedInt32Array::new();
    let mut verts = PackedVector3Array::new();
    let mut normals = PackedVector3Array::new();
    let mut uvs = PackedVector2Array::new();

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

            let unoriented_quad = UnorientedQuad::from(quad);
            for i in &face.tex_coords(RIGHT_HANDED_Y_UP_CONFIG.u_flip_face, true, &unoriented_quad) {
                uvs.push(Vector2::new(i[0], i[1]))
            }
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
    arrays.set(
        mesh::ArrayType::ARRAY_TEX_UV.ord() as usize,
        Variant::from(uvs),
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
