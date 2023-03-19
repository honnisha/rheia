use crate::{
    chunks::{block_info::BlockInfo, block_type::get_block_type_by_id},
    utils::block_mesh::{
        ndshape::ConstShape3u32, visible_block_faces, UnitQuadBuffer, UnorientedQuad,
        RIGHT_HANDED_Y_UP_CONFIG,
    },
};
use godot::prelude::{Array, Gd, Vector3, godot_print};
use godot::{engine::ArrayMesh, prelude::Variant};
use godot::{
    engine::*,
    prelude::{PackedInt32Array, PackedVector2Array, PackedVector3Array},
};
use godot::{obj::EngineEnum, prelude::Vector2};

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

    let steep = 0.03125;
    let uv_scale = Vector2::new(steep, steep);

    let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;

    for (side_index, (group, face)) in buffer.groups.into_iter().zip(faces.into_iter()).enumerate()
    {
        // visible_block_faces_with_voxel_view
        // face is OrientedBlockFace
        // group Vec<UnorientedUnitQuad>
        for quad in group.into_iter() {
            let block_type = get_block_type_by_id(quad.id);

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
                let offset = match block_type.get_uv_offset(side_index as i8) {
                    Some(o) => o,
                    _ => 0,
                };
                let ui_offset = Vector2::new(
                    steep * ((offset % 32) as i32) as f32,
                    steep * ((offset / 32) as f32).floor(),
                );
                uvs.push(Vector2::new(i[0], i[1]) * uv_scale + ui_offset)
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
        Default::default(),
    );
    mesh_ist.regen_normal_maps();
    Some(mesh_ist)
}
