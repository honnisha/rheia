use std::borrow::BorrowMut;

use common::{
    blocks::blocks_storage::BlockType,
    chunks::chunk_position::ChunkPosition,
    physics::{physics::IPhysicsCollider, PhysicsCollider, PhysicsColliderBuilder},
    CHUNK_SIZE,
};
use godot::{
    engine::{Material, MeshInstance3D},
    prelude::*,
};
use ndshape::{ConstShape, ConstShape3u32};

use crate::{
    utils::bridge::{GodotPositionConverter, IntoNetworkVector},
    world::physics::{PhysicsProxy, PhysicsType},
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
    chunk_position: ChunkPosition,
    y: u8,

    pub need_update_geometry: bool,

    collider: Option<PhysicsCollider>,
    colider_builder: Option<PhysicsColliderBuilder>,
}

impl ChunkSection {
    pub fn create(base: Base<Node3D>, material: Gd<Material>, y: u8, chunk_position: ChunkPosition) -> Self {
        let mut mesh = MeshInstance3D::new_alloc();
        mesh.set_name(GString::from(format!("ChunkMesh {}", y)));
        mesh.set_material_overlay(material.clone());

        // Disable while its empty
        mesh.set_process(false);

        Self {
            base,
            mesh,
            chunk_position,
            y,

            need_update_geometry: false,
            collider: None,
            colider_builder: None,
        }
    }

    pub fn get_section_local_position(&self) -> Vector3 {
        Vector3::new(0.0, GodotPositionConverter::get_chunk_y_local(self.y), 0.0)
    }

    pub fn get_section_position(&self) -> Vector3 {
        Vector3::new(
            self.chunk_position.x as f32 * CHUNK_SIZE as f32 - 1_f32,
            GodotPositionConverter::get_chunk_y_local(self.y) - 1_f32,
            self.chunk_position.z as f32 * CHUNK_SIZE as f32 - 1_f32,
        )
    }

    /// Updates the mesh from a separate thread
    pub fn set_new_geometry(&mut self, geometry: Geometry) {
        let mesh = self.mesh.borrow_mut();

        let c = geometry.mesh_ist.get_surface_count();

        // Set active only for sections that conatains vertices
        mesh.set_process(c > 0);

        mesh.set_mesh(geometry.mesh_ist.upcast());

        self.need_update_geometry = true;
        self.colider_builder = geometry.collider_builder
    }

    /// Causes an update in the main thread after the entire chunk has been loaded
    pub fn update_geometry(&mut self, physics: &PhysicsProxy) {
        self.need_update_geometry = false;

        // Remove old collider if exists
        if let Some(collider) = self.collider.take() {
            collider.remove();
        }

        // Set new colider
        if let Some(colider_builder) = self.colider_builder.take() {
            let mut collider = physics.create_collider(
                colider_builder,
                PhysicsType::ChunkMeshCollider(self.chunk_position.clone()),
            );
            let pos = self.get_section_position().clone();
            collider.set_position(pos.to_network());
            self.collider = Some(collider);
        }
    }

    pub fn set_active(&mut self, state: bool) {
        if let Some(c) = self.collider.as_mut() {
            c.set_enabled(state);
        }
    }
}

#[godot_api]
impl INode3D for ChunkSection {
    fn ready(&mut self) {
        let mesh = self.mesh.clone().upcast();
        self.base_mut().add_child(mesh);
    }
}
