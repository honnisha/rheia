use crate::{
    chunks::{block_info::BlockInfo, block_type::get_block_type_by_id},
    utils::block_mesh::{
        ndshape::ConstShape3u32, visible_block_faces, Axis, AxisPermutation, OrientedBlockFace,
        QuadCoordinateConfig, UnitQuadBuffer, UnorientedQuad, RIGHT_HANDED_Y_UP_CONFIG,
    },
};
use godot::prelude::{godot_print, Array, Gd, Vector3};
use godot::{engine::ArrayMesh, prelude::Variant};
use godot::{
    engine::*,
    prelude::{PackedInt32Array, PackedVector2Array, PackedVector3Array},
};
use godot::{obj::EngineEnum, prelude::Vector2};
use ndshape::ConstShape;

// A 16^3 chunk with 1-voxel boundary padding.
pub type ChunkShape = ConstShape3u32<16, 16, 16>;
pub type ChunkBordersShape = ConstShape3u32<18, 18, 18>;

#[allow(dead_code)]
pub fn get_test_sphere() -> [BlockInfo; 5832] {
    let mut b_chunk = [BlockInfo::new(0); 5832];

    for i in 0u32..5832 {
        let [x, y, z] = ChunkBordersShape::delinearize(i);
        b_chunk[i as usize] = match ((x * x + y * y + z * z) as f32).sqrt() < 7.0 {
            true => BlockInfo::new(1),
            _ => BlockInfo::new(0),
        };
    }
    b_chunk
}

pub fn generate_buffer(chunk_data: &[BlockInfo; 5832]) -> UnitQuadBuffer {
    //let b_chunk = get_test_sphere();

    let mut buffer = UnitQuadBuffer::new();
    visible_block_faces(
        chunk_data, //&b_chunk,
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

    // let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;

    let s = QuadCoordinateConfig {
        // Y is always in the V direction when it's not the normal. When Y is the
        // normal, right-handedness determines that we must use Yzx permutations.
        faces: [
            OrientedBlockFace::new(-1, AxisPermutation::Xzy),
            OrientedBlockFace::new(-1, AxisPermutation::Yzx),
            OrientedBlockFace::new(-1, AxisPermutation::Zxy),
            OrientedBlockFace::new(1, AxisPermutation::Xzy),
            OrientedBlockFace::new(1, AxisPermutation::Yzx),
            OrientedBlockFace::new(1, AxisPermutation::Zxy),
        ],
        u_flip_face: Axis::Y,
    };

    for (side_index, (group, face)) in buffer
        .groups
        .into_iter()
        .zip(s.faces.into_iter())
        .enumerate()
    {
        // visible_block_faces_with_voxel_view
        // face is OrientedBlockFace
        // group Vec<UnorientedUnitQuad>
        for quad in group.into_iter() {
            let block_type = get_block_type_by_id(&quad.id);

            indices.extend(face.quad_mesh_indices(verts.len() as i32));
            verts.extend(face.quad_mesh_positions(&quad.into(), 1.0));
            normals.extend(face.quad_mesh_normals());

            let unoriented_quad = UnorientedQuad::from(quad);
            for i in &face.tex_coords(s.u_flip_face, false, &unoriented_quad) {
                let offset = match block_type.get_uv_offset(side_index as i8) {
                    Some(o) => o,
                    _ => 0,
                };
                let ui_offset = Vector2::new(
                    steep * ((offset % 32) as i32) as f32,
                    steep * ((offset / 32) as f32).floor(),
                );
                uvs.push(*i * uv_scale + ui_offset)
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
    Some(mesh_ist)
}
