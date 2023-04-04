use std::borrow::BorrowMut;

use godot::{
    engine::{node::InternalMode, Material, MeshInstance3D},
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
    position: [i32; 3],
}

#[godot_api]
impl Chunk {

    pub fn create_mesh(&mut self, material: &Gd<Material>) {
        let mut mesh = MeshInstance3D::new_alloc();
        mesh.set_name(GodotString::from("ChunkMesh"));

        mesh.set_material_overlay(material.share());

        self.base
            .add_child(mesh.upcast(), true, InternalMode::INTERNAL_MODE_BACK);
        let m = self.base.get_node_as::<MeshInstance3D>("ChunkMesh");
        self.mesh = Some(m);
    }
}

impl Chunk {
    pub fn create(base: Base<Node3D>, chunk_data: [BlockInfo; 4096], position: [i32; 3]) -> Self {
        Chunk {
            base: base,
            chunk_data: chunk_data,
            mesh: None,
            loaded: false,
            position: position,
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    pub fn get_chunk_data(&self) -> &[BlockInfo; 4096] {
        &self.chunk_data
    }

    #[allow(dead_code)]
    pub fn get_block_info(&self, position: [u32; 3]) -> BlockInfo {
        return self.chunk_data[ChunkShape::linearize(position) as usize];
    }

    pub fn get_chunk_position(&self) -> [i32; 3] {
        self.position
    }

    // Get global position from chunk coordinate
    pub fn get_chunk_position_from_coordinate(position: &[i32; 3]) -> Vector3 {
        // -1 because of chunk boundaries
        Vector3::new(
            position[0] as f32 * 16.0 - 1_f32,
            position[1] as f32 * 16.0 - 1_f32,
            position[2] as f32 * 16.0 - 1_f32,
        )
    }

    fn fix_chunk_loc_pos(p: i32) -> i32 {
        if p < 0 {
            return (p + 1_i32) / 16_i32 + -1_i32;
        }
        return p / 16_i32;
    }
    /// Return chunk position from global coordinate
    pub fn get_chunk_pos_by_global(p: &[i32; 3]) -> [i32; 3] {
        [
            Chunk::fix_chunk_loc_pos(p[0]),
            Chunk::fix_chunk_loc_pos(p[1]),
            Chunk::fix_chunk_loc_pos(p[2]),
        ]
    }

    fn fix_loc_pos(p: i32) -> u32 {
        if p < 0 {
            return (15_i32 + ((p + 1_i32) % 16_i32)) as u32;
        }
        return (p % 16_i32) as u32;
    }
    /// Return chunk local position
    /// by global coordinate
    pub fn get_chunk_local_pos_from_global(p: &[i32; 3]) -> [u32; 3] {
        [
            Chunk::fix_loc_pos(p[0]),
            Chunk::fix_loc_pos(p[1]),
            Chunk::fix_loc_pos(p[2]),
        ]
    }

    pub fn set_block(&mut self, global_pos: &[i32; 3], block_info: BlockInfo) {
        let local_pos = Chunk::get_chunk_local_pos_from_global(global_pos);
        let i = ChunkShape::linearize(local_pos) as usize;
        self.chunk_data[i] = block_info;
    }

    pub fn update_mesh(
        &mut self,
        bordered_chunk_data: &[BlockType; 5832],
        texture_mapper: &TextureMapper,
    ) {
        let mesh = generate_chunk_geometry(&texture_mapper, &bordered_chunk_data);

        let m = self.mesh.as_mut().unwrap().borrow_mut();
        m.set_mesh(mesh.upcast());
        m.create_trimesh_collision();
        //m.create_convex_collision(false, false);

        self.loaded = true;
    }
}

#[godot_api]
impl Node3DVirtual for Chunk {
    fn init(base: Base<Node3D>) -> Self {
        Chunk::create(
            base,
            [BlockInfo::new(BlockType::Air); 4096],
            [0_i32, 0_i32, 0_i32],
        )
    }

    fn ready(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::Chunk;

    #[test]
    fn test_get_chunk_pos_by_global() {
        assert_eq!(
            Chunk::get_chunk_pos_by_global(&[0_i32, 1_i32, 20_i32]),
            [0_i32, 0_i32, 1_i32]
        );
        assert_eq!(
            Chunk::get_chunk_pos_by_global(&[-15_i32, -16_i32, -17_i32]),
            [-1_i32, -1_i32, -2_i32]
        );
        assert_eq!(
            Chunk::get_chunk_pos_by_global(&[33_i32, -1_i32, -20_i32]),
            [2_i32, -1_i32, -2_i32]
        );
    }

    #[test]
    fn test_get_chunk_local_pos_from_global() {
        assert_eq!(
            Chunk::get_chunk_local_pos_from_global(&[0_i32, 1_i32, 20_i32]),
            [0_u32, 1_u32, 4_u32]
        );
        assert_eq!(
            Chunk::get_chunk_local_pos_from_global(&[0_i32, -1_i32, -2_i32]),
            [0_u32, 15_u32, 14_u32]
        );
        assert_eq!(
            Chunk::get_chunk_local_pos_from_global(&[-15_i32, -16_i32, -17_i32]),
            [1_u32, 0_u32, 15_u32]
        );
    }
}
