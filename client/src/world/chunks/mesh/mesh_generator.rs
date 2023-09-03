use crate::{
    main_scene::FloatType,
    utils::textures::texture_mapper::TextureMapper,
    world::chunks::godot_chunk_section::{ChunkBordersShape, ChunkDataBordered},
};
use common::blocks::blocks_storage::BlockType;
use godot::prelude::{Array, Gd};
use godot::{engine::ArrayMesh, prelude::Variant};
use godot::{
    engine::*,
    prelude::{PackedInt32Array, PackedVector2Array, PackedVector3Array},
};
use godot::{obj::EngineEnum, prelude::Vector2};
use log::error;
use ndshape::ConstShape;
use parking_lot::RwLockReadGuard;

use super::block_mesh::{visible_block_faces, UnitQuadBuffer, RIGHT_HANDED_Y_UP_CONFIG, UnorientedQuad};

#[allow(dead_code)]
pub fn get_test_sphere() -> ChunkDataBordered {
    let mut b_chunk = [BlockType::Air; ChunkBordersShape::SIZE as usize];

    for i in 0u32..(ChunkBordersShape::SIZE as u32) {
        let [x, y, z] = ChunkBordersShape::delinearize(i);
        b_chunk[i as usize] = match ((x * x + y * y + z * z) as f32).sqrt() < 7.0 {
            true => BlockType::Stone,
            _ => BlockType::Air,
        };
    }
    b_chunk
}

pub fn generate_buffer(chunk_data: &ChunkDataBordered) -> UnitQuadBuffer {
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

pub trait GeometryTrait: Send + Sync {}

pub struct Geometry {
    pub mesh_ist: Gd<ArrayMesh>,
}

unsafe impl Send for Geometry {}
unsafe impl Sync for Geometry {}

pub fn generate_chunk_geometry(
    texture_mapper: &RwLockReadGuard<TextureMapper>,
    chunk_data: &ChunkDataBordered,
) -> Geometry {
    let mut arrays: Array<Variant> = Array::new();
    let mut mesh_ist = ArrayMesh::new();
    arrays.resize(mesh::ArrayType::ARRAY_MAX.ord() as usize);

    let buffer = generate_buffer(chunk_data);

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
            let block_type_info = match quad.block_type.get_block_type_info() {
                Some(e) => e,
                _ => {
                    error!("GENERATE_CHUNK_GEOMETRY cant get block_type_info");
                    panic!();
                }
            };

            indices.extend(face.quad_mesh_indices(verts.len() as i32));
            verts.extend(face.quad_mesh_positions(&quad.into(), 1.0));
            normals.extend(face.quad_mesh_normals());

            let unoriented_quad = UnorientedQuad::from(quad);

            for i in &face.tex_coords(RIGHT_HANDED_Y_UP_CONFIG.u_flip_face, false, &unoriented_quad) {
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
                uvs.push(*i * uv_scale + ui_offset)
            }
        }
    }

    if indices.len() == 0 {
        return Geometry { mesh_ist: mesh_ist };
    }

    arrays.set(mesh::ArrayType::ARRAY_INDEX.ord() as usize, Variant::from(indices));
    arrays.set(mesh::ArrayType::ARRAY_VERTEX.ord() as usize, Variant::from(verts));
    arrays.set(mesh::ArrayType::ARRAY_NORMAL.ord() as usize, Variant::from(normals));
    arrays.set(mesh::ArrayType::ARRAY_TEX_UV.ord() as usize, Variant::from(uvs));

    mesh_ist.add_surface_from_arrays(mesh::PrimitiveType::PRIMITIVE_TRIANGLES, arrays);
    Geometry { mesh_ist: mesh_ist }
}
