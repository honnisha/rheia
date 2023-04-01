use std::borrow::BorrowMut;

use godot::{
    engine::{node::InternalMode, Material, MeshInstance3D, StandardMaterial3D},
    prelude::*,
};
use ndshape::ConstShape;

use crate::{
    utils::{
        mesh::mesh_generator::{generate_chunk_geometry, ChunkShape},
        textures::texture_mapper::TextureMapper,
    },
    world::blocks::blocks_storage::BlockType,
};

use super::block_info::BlockInfo;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct Chunk {
    #[base]
    pub base: Base<Node3D>,
    chunk_data: [BlockInfo; 4096],
    mesh: Option<Gd<MeshInstance3D>>,
    loaded: bool,
}

#[godot_api]
impl Chunk {}

impl Chunk {
    pub fn create(base: Base<Node3D>, chunk_data: [BlockInfo; 4096]) -> Self {
        Chunk {
            base: base,
            chunk_data: chunk_data,
            mesh: None,
            loaded: false,
        }
    }

    pub fn create_mesh(&mut self, chunk_position: &[i32; 3], material: &Gd<StandardMaterial3D>) {
        let mut mesh = MeshInstance3D::new_alloc();
        mesh.set_name(GodotString::from("ChunkMesh"));

        let new_mat = material.duplicate(true).unwrap().cast::<Material>();
        mesh.set_material_overlay(new_mat);

        let global_pos = Chunk::get_chunk_position_from_coordinate(&chunk_position);
        mesh.set_global_position(global_pos);

        self.base
            .add_child(mesh.upcast(), true, InternalMode::INTERNAL_MODE_BACK);
        let m = self.base.get_node_as::<MeshInstance3D>("ChunkMesh");
        self.mesh = Some(m);
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    pub fn get_chunk_position(&self) -> [i32; 3] {
        let p = self.base.get_global_position();
        Chunk::get_chunk_positions_by_coordinate(&[p.x as i32, p.y as i32, p.z as i32])
    }

    pub fn get_chunk_data(&self) -> &[BlockInfo; 4096] {
        &self.chunk_data
    }

    #[allow(dead_code)]
    pub fn get_block_info(&self, position: [u32; 3]) -> BlockInfo {
        return self.chunk_data[ChunkShape::linearize(position) as usize];
    }

    pub fn get_chunk_position_from_coordinate(position: &[i32; 3]) -> Vector3 {
        Vector3::new(
            position[0] as f32 * 16.0,
            position[1] as f32 * 16.0,
            position[2] as f32 * 16.0,
        )
    }

    pub fn get_chunk_positions_by_coordinate(c: &[i32; 3]) -> [i32; 3] {
        [c[0] % 16, c[1] % 16, c[2] % 16]
    }

    fn get_local_from_global(&self, global_pos: &[i32; 3]) -> [u32; 3] {
        let p = self.get_chunk_position();
        [
            (global_pos[0] - (p[0] * 16_i32) as i32) as u32,
            (global_pos[1] - (p[1] * 16_i32) as i32) as u32,
            (global_pos[2] - (p[2] * 16_i32) as i32) as u32,
        ]
    }

    pub fn set_block(&mut self, global_pos: &[i32; 3], block_type: BlockType) {
        let local_pos = self.get_local_from_global(global_pos);
        let i = ChunkShape::linearize(local_pos) as usize;
        self.chunk_data[i] = BlockInfo::new(block_type);
    }

    pub fn update_mesh(&mut self, bordered_chunk_data: &[BlockType; 5832], texture_mapper: &TextureMapper) {
        let mesh = generate_chunk_geometry(&texture_mapper, &bordered_chunk_data);

        let m = self.mesh.as_mut().unwrap().borrow_mut();
        m.set_mesh(mesh.upcast());
        m.create_trimesh_collision();

        self.loaded = true;
    }
}

#[godot_api]
impl Node3DVirtual for Chunk {
    fn init(base: Base<Node3D>) -> Self {
        Chunk::create(base, [BlockInfo::new(BlockType::Air); 4096])
    }

    fn ready(&mut self) {}
}
