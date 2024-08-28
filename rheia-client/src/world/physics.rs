use std::sync::Arc;

use ahash::AHashMap;
use common::{
    chunks::chunk_position::ChunkPosition,
    physics::physics::{PhysicsContainer, PhysicsStaticEntity},
};
use parking_lot::RwLock;

use crate::main_scene::{PhysicsContainerType, PhysicsRigidBodyEntityType, PhysicsStaticEntityType};

#[derive(Clone)]
pub enum PhysicsType {
    ChunkMeshCollider(ChunkPosition),
    EntityCollider(u32),
}

#[derive(Default, Clone)]
pub struct PhysicsProxy {
    physics_container: PhysicsContainerType,
    collider_type_map: Arc<RwLock<AHashMap<usize, PhysicsType>>>,
}

impl PhysicsProxy {
    pub fn step(&self, delta: f32) {
        self.physics_container.step(delta);
    }

    pub fn create_static(&self, collider_type: PhysicsType) -> PhysicsStaticEntityType {
        let collider = self.physics_container.create_static();
        self.collider_type_map
            .write()
            .insert(collider.get_index().unwrap(), collider_type);
        collider
    }

    pub fn create_rigid_body(&self, height: f32, radius: f32, mass: f32) -> PhysicsRigidBodyEntityType {
        self.physics_container.create_rigid_body(height, radius, mass)
    }

    pub fn get_type_by_collider(&self, collider_id: usize) -> Option<PhysicsType> {
        match self.collider_type_map.read().get(&collider_id) {
            Some(c) => Some(c.clone()),
            None => None,
        }
    }
}
