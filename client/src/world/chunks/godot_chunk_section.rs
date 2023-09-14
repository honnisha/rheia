use std::borrow::BorrowMut;

use common::blocks::blocks_storage::BlockType;
use godot::{
    engine::{ArrayMesh, Material, MeshInstance3D},
    prelude::*,
};
use ndshape::{ConstShape, ConstShape3u32};

use crate::world::godot_world::get_default_material;

//pub type ChunkShape = ConstShape3u32<16, 16, 16>;
pub type ChunkBordersShape = ConstShape3u32<18, 18, 18>;

//pub type ChunkData = [BlockInfo; ChunkShape::SIZE as usize];
pub type ChunkDataBordered = [BlockType; ChunkBordersShape::SIZE as usize];

/// Chunk section, one of the chunk column
/// Contains mesh and data of the chunk section blocks
#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct ChunkSection {
    #[base]
    pub(crate) base: Base<Node3D>,
    mesh: Gd<MeshInstance3D>,
}

impl ChunkSection {
    pub fn create(base: Base<Node3D>, material: Gd<Material>) -> Self {
        let mut mesh = MeshInstance3D::new_alloc();
        mesh.set_name(GodotString::from("ChunkMesh"));
        mesh.set_material_overlay(material.share());

        Self { base, mesh }
    }

    pub fn update_mesh(&mut self, new_mesh: Gd<ArrayMesh>) {
        let mesh = self.mesh.borrow_mut();
        //let c = new_mesh.get_surface_count();
        mesh.set_mesh(new_mesh.upcast());
        // println!("update_mesh y:{} surface_count:{}", self.y, c);
    }
}

#[godot_api]
impl NodeVirtual for ChunkSection {
    /// For default godot init; only Self::create is using
    fn init(base: Base<Node3D>) -> Self {
        Self::create(base, get_default_material())
    }

    fn ready(&mut self) {
        self.base.add_child(self.mesh.share().upcast());
    }
}
