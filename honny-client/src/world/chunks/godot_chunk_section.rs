use std::borrow::BorrowMut;

use common::{blocks::blocks_storage::BlockType, chunks::chunk_position::ChunkPosition};
use godot::{
    engine::{Material, MeshInstance3D},
    prelude::*,
};
use ndshape::{ConstShape, ConstShape3u32};
use rapier3d::prelude::ColliderBuilder;

use crate::{
    entities::position::GodotPositionConverter,
    world::{
        godot_world::get_default_material,
        physics_handler::{PhysicsContainer, PhysicsStaticEntity},
    },
};

use super::{mesh::mesh_generator::Geometry, godot_chunk_column::DEFAULT_CHUNK_ACTIVITY};

//pub type ChunkShape = ConstShape3u32<16, 16, 16>;
pub type ChunkBordersShape = ConstShape3u32<18, 18, 18>;

//pub type ChunkData = [BlockInfo; ChunkShape::SIZE as usize];
pub type ChunkDataBordered = [BlockType; ChunkBordersShape::SIZE as usize];

/// One chunk section
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
    active: bool,

    need_sync: bool,
    new_colider: Option<ColliderBuilder>,
}

impl ChunkSection {
    pub fn create(
        base: Base<Node3D>,
        material: Gd<Material>,
        y: u8,
        physics_entity: PhysicsStaticEntity,
        chunk_position: ChunkPosition,
    ) -> Self {
        let mut mesh = MeshInstance3D::new_alloc();
        mesh.set_name(GodotString::from("ChunkMesh"));
        mesh.set_material_overlay(material.clone());

        Self {
            base,
            mesh,
            chunk_position,
            physics_entity,
            y,
            active: DEFAULT_CHUNK_ACTIVITY,

            need_sync: false,
            new_colider: None,
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

    /// Updates the mesh from a separate thread
    pub fn update_mesh(&mut self, geometry: Geometry) {
        let mesh = self.mesh.borrow_mut();
        mesh.set_mesh(geometry.mesh_ist.upcast());

        self.need_sync = true;
        self.new_colider = geometry.collider
    }

    /// Causes an update in the main thread after the entire chunk has been loaded
    pub fn sync(&mut self) {
        if self.need_sync {
            self.need_sync = false;

            // This function causes a thread lock
            self.physics_entity
                .update_collider(std::mem::take(&mut self.new_colider), &self.get_section_position());
            self.new_colider = None;

            if self.physics_entity.has_collider() {
                self.physics_entity.set_enabled(self.active);
            }
        }
    }

    pub fn change_activity(&mut self, active: bool) {
        if self.active != active {
            self.active = active;

            if self.physics_entity.has_collider() {
                if self.chunk_position.x == 0 && self.chunk_position.z == -1 {
                    println!("----------------- update chunk:{} y:{} active:{}", self.chunk_position, self.y, self.active);
                }
                self.physics_entity.set_enabled(self.active);
            }
        }
    }
}

#[godot_api]
impl NodeVirtual for ChunkSection {
    /// For default godot init; only Self::create is using
    fn init(base: Base<Node3D>) -> Self {
        let physics = PhysicsContainer::default();
        Self::create(
            base,
            get_default_material(),
            0,
            physics.create_static(),
            ChunkPosition::new(0, 0),
        )
    }

    fn ready(&mut self) {
        self.base.add_child(self.mesh.clone().upcast());
    }
}
