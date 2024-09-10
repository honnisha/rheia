use std::sync::Arc;

use ahash::AHashMap;
use common::{
    chunks::chunk_position::ChunkPosition,
    physics::{
        physics::{IPhysicsCollider, IPhysicsContainer, RayCastResultNormal},
        PhysicsCollider, PhysicsColliderBuilder, PhysicsContainer, PhysicsRigidBody, QueryFilter,
    },
};
use godot::prelude::*;
use parking_lot::RwLock;

use crate::utils::bridge::IntoNetworkVector;

#[derive(Clone)]
pub enum PhysicsType {
    ChunkMeshCollider(ChunkPosition),
    EntityCollider(u32),
}

#[derive(Default, Clone)]
pub struct PhysicsProxy {
    physics_container: PhysicsContainer,
    collider_type_map: Arc<RwLock<AHashMap<usize, PhysicsType>>>,
}

impl PhysicsProxy {
    pub fn step(&self, delta: f32) {
        self.physics_container.step(delta);
    }

    pub fn create_collider(
        &self,
        collider_builder: PhysicsColliderBuilder,
        collider_type: PhysicsType,
    ) -> PhysicsCollider {
        let collider = self.physics_container.spawn_collider(collider_builder);
        self.collider_type_map
            .write()
            .insert(collider.get_index(), collider_type);
        collider
    }

    pub fn spawn_rigid_body(&self, collider_builder: PhysicsColliderBuilder) -> (PhysicsRigidBody, PhysicsCollider) {
        self.physics_container.spawn_rigid_body(collider_builder)
    }

    pub fn get_type_by_collider(&self, collider_id: &usize) -> Option<PhysicsType> {
        match self.collider_type_map.read().get(collider_id) {
            Some(c) => Some(c.clone()),
            None => None,
        }
    }

    pub fn cast_ray_and_get_normal(
        &self,
        dir: Vector3,
        max_toi: f32,
        from: Vector3,
        filter: QueryFilter,
    ) -> Option<(RayCastResultNormal, PhysicsType)> {
        match self
            .physics_container
            .cast_ray_and_get_normal(dir.to_network(), max_toi, from.to_network(), filter)
        {
            Some(result) => {
                let Some(collider_type) = self.get_type_by_collider(&result.collider_id) else {
                    panic!(
                        "collider_id:{} not found inside physics proxy; collider_type_map is not consistent",
                        result.collider_id
                    )
                };
                Some((result, collider_type))
            }
            None => None,
        }
    }
}

pub fn get_degrees_from_normal(normal: Vector3) -> Vector3 {
    if normal == Vector3::new(0.0, 0.0, -1.0) { return Vector3::new(0.0, 0.0, 0.0) };
    if normal == Vector3::new(-1.0, 0.0, 0.0) { return Vector3::new(0.0, 90.0, 0.0) };
    if normal == Vector3::new(0.0, 0.0, 1.0) { return Vector3::new(0.0, 180.0, 0.0) };
    if normal == Vector3::new(1.0, 0.0, 0.0) { return Vector3::new(0.0, 270.0, 0.0) };

    // Top
    if normal == Vector3::new(0.0, 1.0, 0.0) { return Vector3::new(90.0, 0.0, 0.0) };

    // Down
    if normal == Vector3::new(0.0, -1.0, 0.0) { return Vector3::new(-90.0, 0.0, 0.0) };

    println!("get_degrees_from_normal is not support normal:{normal}");
    return Vector3::new(33.0, 33.0, 33.0);
}
