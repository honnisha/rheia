use crate::{
    chunks::block_info::BlockInfo,
    utils::block_mesh::{
        ndshape::ConstShape3u32, visible_block_faces, MergeVoxel, UnitQuadBuffer, UnorientedQuad,
        Voxel, VoxelVisibility, RIGHT_HANDED_Y_UP_CONFIG,
    },
};
use godot::prelude::{godot_print, Array, Gd, Vector3};
use godot::{engine::ArrayMesh, prelude::Variant};
use godot::{
    engine::*,
    prelude::{PackedInt32Array, PackedVector2Array, PackedVector3Array},
};
use godot::{obj::EngineEnum, prelude::Vector2};

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
pub type ChunkShape = ConstShape3u32<16, 16, 16>;
pub type ChunkBordersShape = ConstShape3u32<18, 18, 18>;

pub fn generate_buffer(chunk_data: &[BlockInfo; 5832]) -> UnitQuadBuffer {
    let mut buffer = UnitQuadBuffer::new();
    visible_block_faces(
        chunk_data,
        &ChunkBordersShape {},
        [0; 3],
        [17; 3],
        &RIGHT_HANDED_Y_UP_CONFIG.faces,
        &mut buffer,
    );
    buffer
}

pub fn generate_chunk_geometry(chunk_data: &[BlockInfo; 5832]) -> Option<Gd<ArrayMesh>> {
    let mut arrays: Array<Variant> = Array::new();
    arrays.resize(mesh::ArrayType::ARRAY_MAX.ord() as usize);

    let buffer = generate_buffer(chunk_data);

    let mut indices = PackedInt32Array::new();
    let mut verts = PackedVector3Array::new();
    let mut normals = PackedVector3Array::new();
    let mut uvs = PackedVector2Array::new();

    let uv_scale = Vector2::new(0.03125, 0.03125);

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
            for i in &face.tex_coords(RIGHT_HANDED_Y_UP_CONFIG.u_flip_face, true, &unoriented_quad)
            {
                uvs.push(Vector2::new(i[0], i[1]) * uv_scale)
            }
        }
    }

    if &indices.len() == &(0 as usize) {
        return None;
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
    Some(mesh_ist)
}
