use std::borrow::BorrowMut;

use common::{
    blocks::blocks_storage::BlockType,
    chunks::chunk_position::ChunkPosition,
    physics::physics::{PhysicsColliderBuilder, PhysicsStaticEntity},
};
use godot::{
    engine::{Material, MeshInstance3D},
    prelude::*,
};
use ndshape::{ConstShape, ConstShape3u32};

use crate::{
    main_scene::{PhysicsColliderBuilderType, PhysicsStaticEntityType},
    utils::bridge::{GodotPositionConverter, IntoGodotVector, IntoNetworkVector},
};

use super::mesh::mesh_generator::Geometry;

//pub type ChunkShape = ConstShape3u32<16, 16, 16>;
pub type ChunkBordersShape = ConstShape3u32<18, 18, 18>;

//pub type ChunkData = [BlockInfo; ChunkShape::SIZE as usize];
pub type ChunkDataBordered = [BlockType; ChunkBordersShape::SIZE as usize];

/// One chunk section
/// Contains mesh and data of the chunk section blocks
#[derive(GodotClass)]
#[class(no_init, base=Node3D)]
pub struct ChunkSection {
    pub(crate) base: Base<Node3D>,
    mesh: Gd<MeshInstance3D>,
    physics_entity: PhysicsStaticEntityType,
    chunk_position: ChunkPosition,
    y: u8,

    pub need_sync: bool,
    new_colider: Option<PhysicsColliderBuilderType>,
}

impl ChunkSection {
    pub fn create(
        base: Base<Node3D>,
        material: Gd<Material>,
        y: u8,
        physics_entity: PhysicsStaticEntityType,
        chunk_position: ChunkPosition,
    ) -> Self {
        let mut mesh = MeshInstance3D::new_alloc();
        mesh.set_name(GString::from(format!("ChunkMesh {}", y)));
        mesh.set_material_overlay(material.clone());

        // Disable while its empty
        mesh.set_process(false);

        Self {
            base,
            mesh,
            chunk_position,
            physics_entity,
            y,

            need_sync: false,
            new_colider: None,
        }
    }

    pub fn get_section_local_position(&self) -> Vector3 {
        Vector3::new(0.0, GodotPositionConverter::get_chunk_y_local(self.y), 0.0)
    }

    pub fn get_section_position(&self) -> Vector3 {
        let mut pos = self.chunk_position.to_godot();
        pos.y = GodotPositionConverter::get_chunk_y_local(self.y);
        pos
    }

    /// Updates the mesh from a separate thread
    pub fn send_to_update_mesh(&mut self, geometry: Geometry) {
        let mesh = self.mesh.borrow_mut();

        let c = geometry.mesh_ist.get_surface_count();

        // Set active onlt for sections that conatains vertices
        mesh.set_process(c > 0);

        mesh.set_mesh(geometry.mesh_ist.upcast());

        self.need_sync = true;
        self.new_colider = geometry.collider
    }

    /// Causes an update in the main thread after the entire chunk has been loaded
    pub fn chunk_section_sync(&mut self) {
        self.need_sync = false;

        let pos = self.get_section_position().clone();
        if let Some(c) = self.new_colider.as_mut() {
            // This function causes a thread lock
            c.update_collider(&mut self.physics_entity, &pos.to_network())
        } else {
            self.physics_entity.remove_collider();
        }
        self.new_colider = None;
    }

    pub fn set_active(&mut self, state: bool) {
        self.physics_entity.set_enabled(state);
    }
}

#[godot_api]
impl INode3D for ChunkSection {
    fn ready(&mut self) {
        let mesh = self.mesh.clone().upcast();
        self.base_mut().add_child(mesh);
    }
}
