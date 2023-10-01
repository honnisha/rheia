use std::borrow::BorrowMut;

use common::{blocks::blocks_storage::BlockType, chunks::chunk_position::ChunkPosition};
use godot::{
    engine::{Material, MeshInstance3D},
    prelude::*,
};
use ndshape::{ConstShape, ConstShape3u32};

use crate::{
    entities::position::GodotPositionConverter,
    world::{
        godot_world::get_default_material,
        physics_handler::{PhysicsContainer, PhysicsStaticEntity},
    },
};

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
    chunk_position: ChunkPosition,
    y: u8,
}

impl ChunkSection {
    pub fn create(base: Base<Node3D>, material: Gd<Material>, y: u8, physics_entity: PhysicsStaticEntity, chunk_position: ChunkPosition) -> Self {
        let mut mesh = MeshInstance3D::new_alloc();
        mesh.set_name(GodotString::from("ChunkMesh"));
        mesh.set_material_overlay(material.share());

        Self {
            base,
            mesh,
            chunk_position,
            physics_entity,
            y,
        }
    }

    pub fn get_section_local_position(&self) -> Vector3 {
        Vector3::new(0.0, GodotPositionConverter::get_chunk_y_local(self.y), 0.0)
    }

    pub fn get_section_position(&self) -> Vector3 {
        let mut pos = GodotPositionConverter::get_chunk_position_vector(&self.chunk_position);
        pos.y = GodotPositionConverter::get_chunk_y_local(self.y);
        pos
    }

    pub fn update_mesh(&mut self, geometry: Geometry) {
        let mesh = self.mesh.borrow_mut();
        mesh.set_mesh(geometry.mesh_ist.upcast());

        self.physics_entity.update_collider(geometry.collider, &self.get_section_position());
    }
}

#[godot_api]
impl NodeVirtual for ChunkSection {
    /// For default godot init; only Self::create is using
    fn init(base: Base<Node3D>) -> Self {
        let physics = PhysicsContainer::default();
        Self::create(base, get_default_material(), 0, physics.create_static(), ChunkPosition::new(0, 0))
    }

    fn ready(&mut self) {
        self.base.add_child(self.mesh.share().upcast());
    }
}
