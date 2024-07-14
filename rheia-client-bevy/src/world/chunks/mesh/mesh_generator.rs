use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_asset::RenderAssetUsages,
        render_resource::PrimitiveTopology,
    },
};
use common::{
    blocks::blocks_storage::BlockType,
    utils::block_mesh::{visible_block_faces, UnitQuadBuffer, UnorientedQuad, RIGHT_HANDED_Y_UP_CONFIG},
};
use ndshape::ConstShape;

use crate::world::chunks::chunk_section::{ChunkBordersShape, ChunkDataBordered};

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

pub fn generate_chunk_geometry(
    //texture_mapper: &RwLockReadGuard<TextureMapper>,
    chunk_data: &ChunkDataBordered,
) -> Mesh {
    let buffer = generate_buffer(chunk_data);

    let num_indices = buffer.num_quads() * 6;
    let num_vertices = buffer.num_quads() * 4;
    let mut indices = Vec::with_capacity(num_indices);
    let mut positions = Vec::with_capacity(num_vertices);
    let mut normals = Vec::with_capacity(num_vertices);
    let mut tex_coords = Vec::with_capacity(num_vertices);
    //let mut uvs = PackedVector2Array::new();

    //let steep = 0.03125;
    //let uv_scale = Vector2::new(steep, steep);

    let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;

    for (_side_index, (group, face)) in buffer.groups.into_iter().zip(faces.into_iter()).enumerate() {
        // visible_block_faces_with_voxel_view
        // face is OrientedBlockFace
        // group Vec<UnorientedUnitQuad>
        for quad in group.into_iter() {
            //let block_type_info = match quad.block_type.get_block_type_info() {
            //    Some(e) => e,
            //    _ => {
            //        error!("GENERATE_CHUNK_GEOMETRY cant get block_type_info");
            //        panic!();
            //    }
            //};

            indices.extend_from_slice(&face.quad_mesh_indices(positions.len() as i32));

            let voxel_size = 1.0;
            let v = face.quad_corners(&quad.into()).map(|c| {
                let v3 = voxel_size * c.as_vec3();
                bevy::prelude::Vec3::new(v3.x, v3.y, v3.z)
            });
            positions.extend_from_slice(&v);

            let v3 = face.signed_normal().as_vec3();
            normals.extend_from_slice(&[bevy::prelude::Vec3::new(v3.x, v3.y, v3.z); 4]);

            let unoriented_quad = UnorientedQuad::from(quad);
            tex_coords.extend_from_slice(&face.tex_coords(
                RIGHT_HANDED_Y_UP_CONFIG.u_flip_face,
                true,
                &unoriented_quad,
            ));

            //for i in &face.tex_coords(RIGHT_HANDED_Y_UP_CONFIG.u_flip_face, false, &unoriented_quad) {
            //    let offset = match texture_mapper.get_uv_offset(block_type_info, side_index as i8) {
            //        //let offset = match block_type.get_uv_offset(side_index as i8) {
            //        Some(o) => o,
            //        _ => {
            //            error!(
            //                "GENERATE_CHUNK_GEOMETRY cant find offset for block type: {}",
            //                block_type_info
            //            );
            //            panic!();
            //        }
            //    };
            //    let ui_offset = Vector2::new(
            //        steep * ((offset % 32) as i32) as FloatType,
            //        steep * ((offset / 32) as f32).floor() as FloatType,
            //    );
            //    uvs.push(*i * uv_scale + ui_offset)
            //}
        }
    }

    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, VertexAttributeValues::Float32x3(positions));
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, VertexAttributeValues::Float32x3(normals));
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, tex_coords);
    render_mesh.insert_indices(Indices::U32(indices.clone()));

    render_mesh
}
