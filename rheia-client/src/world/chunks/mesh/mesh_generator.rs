use crate::{
    scenes::main_scene::FloatType,
    utils::{bridge::IntoNetworkVector, textures::texture_mapper::TextureMapper},
    world::{
        block_storage::BlockStorage,
        chunks::chunk_section::{ChunkBordersShape, ChunkColliderDataBordered},
    },
};
use common::{
    CHUNK_SIZE,
    blocks::{chunk_collider_info::ChunkColliderInfo, voxel_visibility::VoxelVisibility},
    chunks::{chunk_data::BlockDataInfo, position::Vector3 as NetworkVector3},
    utils::block_mesh::{
        QuadBuffer, RIGHT_HANDED_Y_UP_CONFIG, UnorientedQuad,
        buffer::UnitQuadBuffer,
        greedy::{GreedyQuadsBuffer, greedy_quads},
        visible_block_faces,
    },
};
use godot::{
    classes::{
        ArrayMesh,
        mesh::{ArrayType, PrimitiveType},
    },
    obj::{EngineEnum, NewGd},
    prelude::{Array, Gd, PackedInt32Array, PackedVector2Array, PackedVector3Array, Variant, Vector2, Vector3},
};
use ndshape::ConstShape;
use physics::{PhysicsColliderBuilder, physics::IPhysicsColliderBuilder};

pub(crate) fn _get_test_sphere(radius: f32, block_info: BlockDataInfo) -> ChunkColliderDataBordered {
    let mut b_chunk = [ChunkColliderInfo::create(VoxelVisibility::Opaque, None); ChunkBordersShape::SIZE as usize];

    for i in 0u32..(ChunkBordersShape::SIZE as u32) {
        let [x, y, z] = ChunkBordersShape::delinearize(i);
        b_chunk[i as usize] = match ((x * x + y * y + z * z) as f32).sqrt() < radius {
            true => ChunkColliderInfo::create(VoxelVisibility::Opaque, Some(block_info.clone())),
            _ => ChunkColliderInfo::create(VoxelVisibility::Empty, None),
        };
    }
    b_chunk
}

pub fn generate_buffer(chunk_collider_data: &ChunkColliderDataBordered) -> UnitQuadBuffer {
    let mut buffer = UnitQuadBuffer::new();
    visible_block_faces(
        chunk_collider_data,
        &ChunkBordersShape {},
        [0; 3],
        [CHUNK_SIZE as u32 + 1; 3],
        &RIGHT_HANDED_Y_UP_CONFIG.faces,
        &mut buffer,
    );
    buffer
}

pub fn _generate_buffer_greedy(chunk_collider_data: &ChunkColliderDataBordered) -> QuadBuffer {
    let mut buffer = GreedyQuadsBuffer::new(chunk_collider_data.len());
    greedy_quads(
        chunk_collider_data,
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
    pub collider_builder: Option<PhysicsColliderBuilder>,
}

unsafe impl Send for Geometry {}
unsafe impl Sync for Geometry {}

pub fn generate_chunk_geometry(
    texture_mapper: &TextureMapper,
    chunk_collider_data: &ChunkColliderDataBordered,
    block_storage: &BlockStorage,
) -> Geometry {
    // let chunk_collider_data = &_get_test_sphere(8.0, BlockInfo::create(1, None));

    let mut arrays: Array<Variant> = Array::new();
    arrays.resize(ArrayType::MAX.ord() as usize, &Variant::nil());

    let buffer = generate_buffer(chunk_collider_data);

    let mut indices = PackedInt32Array::new();
    let mut verts = PackedVector3Array::new();
    let mut normals = PackedVector3Array::new();
    let mut uvs = PackedVector2Array::new();

    let steep = 0.03125;
    let uv_scale = Vector2::new(steep, steep);

    let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;

    let mut collider_indices: Vec<[u32; 3]> = Default::default();
    let mut collider_verts: Vec<NetworkVector3> = Default::default();

    for (side_index, (group, face)) in buffer.groups.into_iter().zip(faces.into_iter()).enumerate() {
        // visible_block_faces_with_voxel_view
        // face is OrientedBlockFace
        // group Vec<UnorientedUnitQuad>
        for quad in group.into_iter() {
            let i = face.quad_mesh_indices(verts.len() as i32);
            indices.extend(i);

            // Collider
            collider_indices.push([i[0] as u32, i[1] as u32, i[2] as u32]);
            collider_indices.push([i[3] as u32, i[4] as u32, i[5] as u32]);

            let voxel_size = 1.0;
            let v = face.quad_corners(&quad.clone().into(), true).map(|c| {
                // magic: Offset -1 because of chunk mesh one block boundary
                let vert_pos = Vector3::new(c.x as f32, c.y as f32, c.z as f32) - Vector3::new(1.0, 1.0, 1.0);

                collider_verts.push(vert_pos.to_network());
                vert_pos * voxel_size
            });
            verts.extend(v);

            let n = face.signed_normal();
            normals.extend([Vector3::new(n.x as f32, n.y as f32, n.z as f32); 4]);

            let block_info = quad
                .block_info
                .expect("GENERATE_CHUNK_GEOMETRY block info is not found");
            let block_type = block_storage
                .get(&block_info.get_id())
                .expect("GENERATE_CHUNK_GEOMETRY block type is not found");

            let unoriented_quad = UnorientedQuad::from(quad);
            for i in &face.tex_coords_godot(RIGHT_HANDED_Y_UP_CONFIG.u_flip_face, false, &unoriented_quad) {
                let Some(offset) = texture_mapper.get_uv_offset(block_type, side_index as i8) else {
                    continue;
                };
                let ui_offset = Vector2::new(
                    steep * ((offset % 32) as i32) as FloatType,
                    steep * ((offset / 32) as f32).floor() as FloatType,
                );
                uvs.push(Vector2::new(i[0], i[1]) * uv_scale + ui_offset)
            }
        }
    }

    let len = indices.len();
    arrays.set(ArrayType::INDEX.ord() as usize, &Variant::from(indices));
    arrays.set(ArrayType::VERTEX.ord() as usize, &Variant::from(verts));
    arrays.set(ArrayType::NORMAL.ord() as usize, &Variant::from(normals));
    arrays.set(ArrayType::TEX_UV.ord() as usize, &Variant::from(uvs));

    let mut collider_builder: Option<PhysicsColliderBuilder> = None;

    let mut mesh_ist = ArrayMesh::new_gd();
    if len > 0 {
        mesh_ist.add_surface_from_arrays(PrimitiveType::TRIANGLES, &arrays);

        collider_builder = Some(PhysicsColliderBuilder::trimesh(collider_verts, collider_indices));
    }

    Geometry {
        mesh_ist,
        collider_builder,
    }
}
