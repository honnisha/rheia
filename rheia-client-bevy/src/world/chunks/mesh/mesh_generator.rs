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
    utils::block_mesh::{
        greedy::{greedy_quads, GreedyQuadsBuffer},
        visible_block_faces, QuadBuffer, UnitQuadBuffer, UnorientedQuad, RIGHT_HANDED_Y_UP_CONFIG,
    },
    CHUNK_SIZE,
};
use ndshape::ConstShape;

use crate::world::chunks::chunk_section::{ChunkBordersShape, ChunkDataBordered};

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

pub fn generate_buffer_greedy(chunk_data: &ChunkDataBordered) -> QuadBuffer {
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

pub fn generate_chunk_geometry(
    //texture_mapper: &RwLockReadGuard<TextureMapper>,
    chunk_data: &ChunkDataBordered,
) -> Mesh {
    let buffer = generate_buffer_greedy(chunk_data);

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
            indices.extend_from_slice(&face.quad_mesh_indices(positions.len() as u32));
            positions.extend_from_slice(&face.quad_mesh_positions(&quad.into(), 1.0));
            normals.extend_from_slice(&face.quad_mesh_normals());
            let unoriented_quad = UnorientedQuad::from(quad);
            tex_coords.extend_from_slice(&face.tex_coords(
                RIGHT_HANDED_Y_UP_CONFIG.u_flip_face,
                true,
                &unoriented_quad,
            ));
        }
    }

    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, VertexAttributeValues::Float32x3(positions));
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, VertexAttributeValues::Float32x3(normals));
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, tex_coords);
    render_mesh.insert_indices(Indices::U32(indices.clone()));

    render_mesh
}
