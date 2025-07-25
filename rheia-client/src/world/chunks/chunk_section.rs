use std::borrow::BorrowMut;

use super::{mesh::mesh_generator::Geometry, objects_container::ObjectsContainer};
use crate::{
    utils::bridge::{GodotPositionConverter, IntoNetworkVector},
    world::physics::{PhysicsProxy, PhysicsType},
};
use common::{
    blocks::chunk_collider_info::ChunkColliderInfo, chunks::chunk_position::ChunkPosition, CHUNK_SIZE,
    CHUNK_SIZE_BOUNDARY,
};
use godot::{
    classes::{Material, MeshInstance3D},
    prelude::*,
};
use ndshape::{ConstShape, ConstShape3u32};
use physics::PhysicsColliderBuilder;
use physics::{
    physics::{IPhysicsCollider, IPhysicsColliderBuilder},
    PhysicsCollider,
};

const TRANSPARENCY_SPEED: f32 = 5.0;

//pub type ChunkShape = ConstShape3u32<CHUNK_SIZE_BOUNDARY, CHUNK_SIZE_BOUNDARY, CHUNK_SIZE_BOUNDARY>;
pub type ChunkBordersShape = ConstShape3u32<CHUNK_SIZE_BOUNDARY, CHUNK_SIZE_BOUNDARY, CHUNK_SIZE_BOUNDARY>;

//pub type ChunkData = [BlockInfo; ChunkShape::SIZE as usize];
pub type ChunkColliderDataBordered = [ChunkColliderInfo; ChunkBordersShape::SIZE as usize];

/// One chunk section
/// Contains mesh and data of the chunk section blocks
#[derive(GodotClass)]
#[class(no_init, tool, base=Node3D)]
pub struct ChunkSection {
    pub(crate) base: Base<Node3D>,

    mesh: Gd<MeshInstance3D>,
    objects_container: Gd<ObjectsContainer>,

    chunk_position: ChunkPosition,
    y: u8,

    need_update_geometry: bool,

    collider: Option<PhysicsCollider>,
    colider_builder: Option<PhysicsColliderBuilder>,

    set_geometry_first_time: bool,
    transparancy: f32,
}

impl ChunkSection {
    pub fn create(base: Base<Node3D>, material: Gd<Material>, y: u8, chunk_position: ChunkPosition) -> Self {
        let mut mesh = MeshInstance3D::new_alloc();
        mesh.set_name(&format!("ChunkMesh {}", y));
        mesh.set_material_override(&material);

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

            objects_container: ObjectsContainer::new_alloc(),
            set_geometry_first_time: false,
            transparancy: 1.0,
        }
    }

    pub fn _get_chunk_position(&self) -> &ChunkPosition {
        &self.chunk_position
    }

    pub fn get_section_local_position(&self) -> Vector3 {
        Vector3::new(0.0, GodotPositionConverter::get_chunk_y_local(self.y), 0.0)
    }

    pub fn get_section_position(&self) -> Vector3 {
        Vector3::new(
            self.chunk_position.x as f32 * CHUNK_SIZE as f32,
            GodotPositionConverter::get_chunk_y_local(self.y),
            self.chunk_position.z as f32 * CHUNK_SIZE as f32,
        )
    }

    pub fn get_objects_container_mut(&mut self) -> &mut Gd<ObjectsContainer> {
        &mut self.objects_container
    }

    /// Updates the mesh from a separate thread
    ///
    /// `update_geometry` must be called after
    pub fn set_new_geometry(&mut self, geometry: Geometry) {
        let mesh = self.mesh.borrow_mut();

        let c = geometry.mesh_ist.get_surface_count();

        // Set active only for sections that conatains vertices
        let has_mesh = c > 0;
        mesh.set_process(has_mesh);

        mesh.set_mesh(&geometry.mesh_ist);

        self.need_update_geometry = true;
        self.colider_builder = geometry.collider_builder;

        if has_mesh && !self.set_geometry_first_time {
            self.set_geometry_first_time = true;
            self.transparancy = 1.0;
            mesh.set_transparency(self.transparancy);
        }
    }

    pub fn is_geometry_update_needed(&self) -> bool {
        self.need_update_geometry
    }

    /// Causes an update in the main thread after the entire chunk has been loaded
    pub fn update_geometry(&mut self, physics: &PhysicsProxy) {
        self.need_update_geometry = false;

        // Set or create new colider
        if let Some(colider_builder) = self.colider_builder.take() {
            if let Some(collider) = self.collider.as_mut() {
                collider.set_shape(colider_builder.get_shape());
            } else {
                let mut collider = physics.create_collider(
                    colider_builder,
                    Some(PhysicsType::ChunkMeshCollider(self.chunk_position.clone())),
                );
                let pos = self.get_section_position().clone();
                collider.set_position(pos.to_network());
                self.collider = Some(collider);
            }
        } else {
            // Remove old collider if exists
            if let Some(mut collider) = self.collider.take() {
                collider.remove();
            }
        }
    }

    #[allow(dead_code)]
    pub fn set_active(&mut self, state: bool) {
        if let Some(c) = self.collider.as_mut() {
            c.set_enabled(state);
        }
    }

    pub fn destory(&mut self) {
        self.objects_container.bind_mut().destory();
    }
}

#[godot_api]
impl INode3D for ChunkSection {
    fn ready(&mut self) {
        let mesh = self.mesh.clone();
        self.base_mut().add_child(&mesh);

        let objects_container = self.objects_container.clone();
        self.base_mut().add_child(&objects_container);
    }

    fn process(&mut self, delta: f64) {
        if self.transparancy > 0.0 {
            let obj = self.mesh.borrow_mut();
            self.transparancy -= TRANSPARENCY_SPEED * delta as f32;
            obj.set_transparency(self.transparancy);
        }
    }
}
