use crate::{
    main_scene::{FloatType, PhysicsColliderBuilderType},
    utils::textures::texture_mapper::TextureMapper,
    world::chunks::chunk_section::{ChunkBordersShape, ChunkDataBordered},
};
use common::{
    blocks::blocks_storage::BlockType,
    physics::physics::PhysicsColliderBuilder,
    utils::block_mesh::{
        greedy::{greedy_quads, GreedyQuadsBuffer},
        QuadBuffer,
    },
};
use common::{
    utils::block_mesh::{buffer::UnitQuadBuffer, visible_block_faces, UnorientedQuad, RIGHT_HANDED_Y_UP_CONFIG},
    CHUNK_SIZE,
};
use godot::{
    classes::mesh::PrimitiveType,
    engine::{mesh::ArrayType, ArrayMesh},
    prelude::{Variant, Vector2, Vector3},
};
use godot::{
    obj::EngineEnum,
    prelude::{PackedInt32Array, PackedVector2Array, PackedVector3Array},
};
use godot::{
    obj::NewGd,
    prelude::{Array, Gd},
};
use log::error;
use ndshape::ConstShape;
use parking_lot::RwLockReadGuard;

#[allow(dead_code)]
pub fn get_test_sphere(radius: f32) -> ChunkDataBordered {
    let mut b_chunk = [BlockType::Air; ChunkBordersShape::SIZE as usize];

    for i in 0u32..(ChunkBordersShape::SIZE as u32) {
        let [x, y, z] = ChunkBordersShape::delinearize(i);
        b_chunk[i as usize] = match ((x * x + y * y + z * z) as f32).sqrt() < radius {
            true => BlockType::Stone,
            _ => BlockType::Air,
        };
    }
    b_chunk
}

pub fn generate_buffer(chunk_data: &ChunkDataBordered) -> UnitQuadBuffer {
    //let b_chunk = get_test_sphere(7.0);

    let mut buffer = UnitQuadBuffer::new();
    visible_block_faces(
        chunk_data, //&b_chunk,
        &ChunkBordersShape {},
        [0; 3],
        [CHUNK_SIZE as u32 + 1; 3],
        &RIGHT_HANDED_Y_UP_CONFIG.faces,
        &mut buffer,
    );
    buffer
}

pub fn _generate_buffer_greedy(chunk_data: &ChunkDataBordered) -> QuadBuffer {
    let mut buffer = GreedyQuadsBuffer::new(chunk_data.len());
    greedy_quads(
        chunk_data, //&b_chunk,
        &ChunkBordersShape {},
        [0; 3],
        [CHUNK_SIZE as u32 + 1; 3],
        &RIGHT_HANDED_Y_UP_CONFIG.faces,
        &mut buffer,
    );
    buffer.quads
}

pub struct Geometry {
    pub mesh_ist: Gd<ArrayMesh>,
    pub collider: Option<PhysicsColliderBuilderType>,
}

impl Geometry {}

unsafe impl Send for Geometry {}
unsafe impl Sync for Geometry {}

pub fn generate_chunk_geometry(
    texture_mapper: &RwLockReadGuard<TextureMapper>,
    chunk_data: &ChunkDataBordered,
) -> Geometry {
    let mut arrays: Array<Variant> = Array::new();
    arrays.resize(ArrayType::MAX.ord() as usize, &Variant::nil());

    let buffer = generate_buffer(chunk_data);

    let mut collider_builder = PhysicsColliderBuilderType::create();

    let mut indices = PackedInt32Array::new();
    let mut verts = PackedVector3Array::new();
    let mut normals = PackedVector3Array::new();
    let mut uvs = PackedVector2Array::new();

    let steep = 0.03125;
    let uv_scale = Vector2::new(steep, steep);

    let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;

    for (side_index, (group, face)) in buffer.groups.into_iter().zip(faces.into_iter()).enumerate() {
        // visible_block_faces_with_voxel_view
        // face is OrientedBlockFace
        // group Vec<UnorientedUnitQuad>
        for quad in group.into_iter() {
            let i = face.quad_mesh_indices(verts.len() as i32);
            indices.extend(i);
            collider_builder.push_indexes([i[0] as u32, i[1] as u32, i[2] as u32]);
            collider_builder.push_indexes([i[3] as u32, i[4] as u32, i[5] as u32]);

            let voxel_size = 1.0;
            let v = face.quad_corners(&quad.into(), true).map(|c| {
                collider_builder.push_verts(c.x as f32, c.y as f32, c.z as f32);
                Vector3::new(c.x as f32, c.y as f32, c.z as f32) * voxel_size
            });
            verts.extend(v);

            let n = face.signed_normal();
            normals.extend([Vector3::new(n.x as f32, n.y as f32, n.z as f32); 4]);

            let block_type_info = match quad.block_type.get_block_type_info() {
                Some(e) => e,
                _ => {
                    error!("GENERATE_CHUNK_GEOMETRY cant get block_type_info");
                    panic!();
                }
            };
            let unoriented_quad = UnorientedQuad::from(quad);
            for i in &face.tex_coords_godot(RIGHT_HANDED_Y_UP_CONFIG.u_flip_face, false, &unoriented_quad) {
                let offset = match texture_mapper.get_uv_offset(block_type_info, side_index as i8) {
                    //let offset = match block_type.get_uv_offset(side_index as i8) {
                    Some(o) => o,
                    _ => {
                        error!(
                            "GENERATE_CHUNK_GEOMETRY cant find offset for block type: {}",
                            block_type_info
                        );
                        panic!();
                    }
                };
                let ui_offset = Vector2::new(
                    steep * ((offset % 32) as i32) as FloatType,
                    steep * ((offset / 32) as f32).floor() as FloatType,
                );
                uvs.push(Vector2::new(i[0], i[1]) * uv_scale + ui_offset)
            }
        }
    }

    let collider: Option<PhysicsColliderBuilderType> = if collider_builder.len() == 0 {
        None
    } else {
        collider_builder.compile();
        Some(collider_builder)
    };

    let len = indices.len();
    arrays.set(ArrayType::INDEX.ord() as usize, Variant::from(indices));
    arrays.set(ArrayType::VERTEX.ord() as usize, Variant::from(verts));
    arrays.set(ArrayType::NORMAL.ord() as usize, Variant::from(normals));
    arrays.set(ArrayType::TEX_UV.ord() as usize, Variant::from(uvs));

    let mut mesh_ist = ArrayMesh::new_gd();
    if len > 0 {
        mesh_ist.add_surface_from_arrays(PrimitiveType::TRIANGLES, arrays);
    }

    Geometry { mesh_ist, collider }
}
