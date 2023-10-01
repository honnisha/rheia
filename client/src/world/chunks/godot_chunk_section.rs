use std::borrow::BorrowMut;

use common::blocks::blocks_storage::BlockType;
use godot::{
    engine::{ArrayMesh, Material, MeshInstance3D},
    prelude::*,
};
use ndshape::{ConstShape, ConstShape3u32};

use crate::world::{godot_world::get_default_material, physics_handler::{PhysicsStaticEntity, PhysicsContainer}};

use super::mesh::mesh_generator::Geometry;

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
    physics_entity: PhysicsStaticEntity,
}

impl ChunkSection {
    pub fn create(base: Base<Node3D>, material: Gd<Material>, physics_entity: PhysicsStaticEntity) -> Self {
        let mut mesh = MeshInstance3D::new_alloc();
        mesh.set_name(GodotString::from("ChunkMesh"));
        mesh.set_material_overlay(material.share());

        Self { base, mesh, physics_entity }
    }

    pub fn update_mesh(&mut self, geometry: Geometry) {
        let mesh = self.mesh.borrow_mut();
        //let c = new_mesh.get_surface_count();
        mesh.set_mesh(geometry.mesh_ist.upcast());
        // println!("update_mesh y:{} surface_count:{}", self.y, c);
    }
}

#[godot_api]
impl NodeVirtual for ChunkSection {
    /// For default godot init; only Self::create is using
    fn init(base: Base<Node3D>) -> Self {
        let mut physics = PhysicsContainer::default();
        Self::create(base, get_default_material(), PhysicsStaticEntity::new(&physics))
    }

    fn ready(&mut self) {
        self.base.add_child(self.mesh.share().upcast());
    }
}
