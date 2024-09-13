use common::chunks::position::Vector3;
use nalgebra::Point3;
use parking_lot::{MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
use rapier3d::prelude::*;
use std::sync::Arc;

use crate::physics::{IPhysicsContainer, RayCastResultNormal};

use super::{
    bridge::IntoNaVector3, collider::RapierPhysicsCollider, collider_builder::RapierPhysicsColliderBuilder,
    controller::RapierPhysicsController, query_filter::RapierQueryFilter,
};

#[derive(Clone)]
pub struct RapierPhysicsContainer {
    pub(crate) controller: Arc<RwLock<RapierPhysicsController>>,
    pub(crate) rigid_body_set: Arc<RwLock<RigidBodySet>>,
    pub(crate) collider_set: Arc<RwLock<ColliderSet>>,
    pub(crate) query_pipeline: Arc<RwLock<QueryPipeline>>,
    pub(crate) island_manager: Arc<RwLock<IslandManager>>,
}

impl RapierPhysicsContainer {
    pub fn get_collider(&self, collider_handle: &ColliderHandle) -> Option<MappedRwLockReadGuard<'_, Collider>> {
        RwLockReadGuard::try_map(self.collider_set.read(), |p| match p.get(*collider_handle) {
            Some(c) => Some(c),
            None => None,
        })
        .ok()
    }

    pub fn get_collider_mut(&self, collider_handle: &ColliderHandle) -> Option<MappedRwLockWriteGuard<'_, Collider>> {
        RwLockWriteGuard::try_map(self.collider_set.write(), |p| match p.get_mut(*collider_handle) {
            Some(c) => Some(c),
            None => None,
        })
        .ok()
    }

    pub fn get_rigid_body(&self, rigid_handle: &RigidBodyHandle) -> Option<MappedRwLockReadGuard<RigidBody>> {
        RwLockReadGuard::try_map(self.rigid_body_set.read(), |p| match p.get(*rigid_handle) {
            Some(c) => Some(c),
            None => None,
        })
        .ok()
    }

    pub fn get_rigid_body_mut(&mut self, rigid_handle: &RigidBodyHandle) -> Option<MappedRwLockWriteGuard<RigidBody>> {
        RwLockWriteGuard::try_map(self.rigid_body_set.write(), |p| match p.get_mut(*rigid_handle) {
            Some(c) => Some(c),
            None => None,
        })
        .ok()
    }
}

impl Default for RapierPhysicsContainer {
    fn default() -> Self {
        let rapier_physics_container = Self {
            controller: Arc::new(RwLock::new(RapierPhysicsController::create())),
            rigid_body_set: Arc::new(RwLock::new(RigidBodySet::new())),
            collider_set: Arc::new(RwLock::new(ColliderSet::new())),
            query_pipeline: Arc::new(RwLock::new(QueryPipeline::new())),
            island_manager: Arc::new(RwLock::new(IslandManager::new())),
        };
        rapier_physics_container
    }
}

impl<'a> IPhysicsContainer<RapierPhysicsCollider, RapierPhysicsColliderBuilder, RapierQueryFilter<'a>>
    for RapierPhysicsContainer
{
    fn step(&self, delta: f32) {
        self.controller.as_ref().write().step(delta, self);
    }

    fn spawn_collider(&self, mut collider_builder: RapierPhysicsColliderBuilder) -> RapierPhysicsCollider {
        let collider = std::mem::take(&mut collider_builder.builder);
        let collider_handle = self.collider_set.write().insert(collider);
        RapierPhysicsCollider::create(&self, collider_handle)
    }

    // https://docs.godotengine.org/en/stable/classes/class_node3d.html#class-node3d-property-rotation
    fn cast_ray_and_get_normal(
        &self,
        dir: Vector3,
        max_toi: f32,
        origin: Vector3,
        filter: RapierQueryFilter,
    ) -> Option<RayCastResultNormal> {
        let origin = Point3::new(origin.x, origin.y, origin.z);
        let direction = dir.to_na();

        let ray = Ray::new(origin, direction);

        let solid = true;

        let pipeline = self.query_pipeline.read();
        if let Some((handle, ray_intersection)) = pipeline.cast_ray_and_get_normal(
            &self.rigid_body_set.read(),
            &self.collider_set.read(),
            &ray,
            max_toi,
            solid,
            filter.filter,
        ) {
            let point = ray.point_at(ray_intersection.time_of_impact);
            let result = RayCastResultNormal {
                collider_id: handle.into_raw_parts().0 as usize,
                point: Vector3::new(point.x, point.y, point.z),
                normal: Vector3::new(
                    ray_intersection.normal.x,
                    ray_intersection.normal.y,
                    ray_intersection.normal.z,
                ),
            };
            return Some(result);
        }
        return None;
    }
}
