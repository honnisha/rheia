use std::sync::Arc;

use ahash::AHashMap;
use common::chunks::chunk_position::ChunkPosition;
use godot::prelude::*;
use parking_lot::RwLock;
use physics::physics::IPhysicsCollider;
use physics::physics::IPhysicsContainer;
use physics::physics::RayCastResultNormal;
use physics::PhysicsCollider;
use physics::PhysicsColliderBuilder;
use physics::PhysicsContainer;
use physics::PhysicsShape;
use physics::QueryFilter;

use crate::controller::camera_controller::RayDirection;
use crate::utils::bridge::IntoGodotVector;
use crate::utils::bridge::IntoNetworkVector;

#[derive(Clone, Copy, Debug, PartialEq)]
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
        collider_type: Option<PhysicsType>,
    ) -> PhysicsCollider {
        let collider = self.physics_container.spawn_collider(collider_builder);
        if let Some(c) = collider_type {
            self.collider_type_map.write().insert(collider.get_index(), c);
        }
        collider
    }

    pub fn get_type_by_collider(&self, collider_id: &usize) -> Option<PhysicsType> {
        match self.collider_type_map.read().get(collider_id) {
            Some(c) => Some(c.clone()),
            None => None,
        }
    }

    pub fn cast_ray(
        &self,
        ray_direction: RayDirection,
        filter: QueryFilter,
    ) -> Option<(RayCastResultNormal, PhysicsType)> {
        match self.physics_container.cast_ray(
            ray_direction.from.to_network(),
            ray_direction.dir.to_network(),
            ray_direction.max_toi,
            filter,
        ) {
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

    pub fn cast_shape(&self, shape: PhysicsShape, ray_direction: RayDirection, filter: QueryFilter) -> Option<Vector3> {
        match self.physics_container.cast_shape(
            shape,
            ray_direction.from.to_network(),
            ray_direction.dir.to_network(),
            ray_direction.max_toi,
            filter,
        ) {
            Some(result) => {
                Some(result.point.to_godot())
            },
            None => None,
        }
    }
}

pub fn get_degrees_from_normal(normal: Vector3) -> Vector3 {
    if normal == Vector3::new(0.0, 0.0, -1.0) {
        return Vector3::new(0.0, 0.0, 0.0);
    };
    if normal == Vector3::new(-1.0, 0.0, 0.0) {
        return Vector3::new(0.0, 90.0, 0.0);
    };
    if normal == Vector3::new(0.0, 0.0, 1.0) {
        return Vector3::new(0.0, 180.0, 0.0);
    };
    if normal == Vector3::new(1.0, 0.0, 0.0) {
        return Vector3::new(0.0, 270.0, 0.0);
    };

    // Top
    if normal == Vector3::new(0.0, 1.0, 0.0) {
        return Vector3::new(90.0, 0.0, 0.0);
    };

    // Down
    if normal == Vector3::new(0.0, -1.0, 0.0) {
        return Vector3::new(-90.0, 0.0, 0.0);
    };

    log::error!("get_degrees_from_normal is not support normal:{normal}");
    return Vector3::new(33.0, 33.0, 33.0);
}
