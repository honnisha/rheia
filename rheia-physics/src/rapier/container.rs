use common::chunks::position::Vector3;
use nalgebra::Point3;
use parking_lot::{MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
use rapier3d::{parry::query::ShapeCastOptions, prelude::*};
use std::sync::Arc;

use crate::physics::{IPhysicsContainer, RayCastResultNormal, ShapeCastResult};

use super::{
    bridge::IntoNaVector3,
    collider::{RapierPhysicsCollider, RapierPhysicsShape},
    collider_builder::RapierPhysicsColliderBuilder,
    controller::RapierPhysicsController,
    query_filter::RapierQueryFilter,
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

    pub fn get_rigid_body(&self, rigid_handle: &RigidBodyHandle) -> Option<MappedRwLockReadGuard<'_, RigidBody>> {
        RwLockReadGuard::try_map(self.rigid_body_set.read(), |p| match p.get(*rigid_handle) {
            Some(c) => Some(c),
            None => None,
        })
        .ok()
    }

    pub fn get_rigid_body_mut(&mut self, rigid_handle: &RigidBodyHandle) -> Option<MappedRwLockWriteGuard<'_, RigidBody>> {
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

impl<'a>
    IPhysicsContainer<RapierPhysicsShape, RapierPhysicsCollider, RapierPhysicsColliderBuilder, RapierQueryFilter<'a>>
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
    fn cast_ray(
        &self,
        origin: Vector3,
        dir: Vector3,
        max_toi: f32,
        filter: RapierQueryFilter,
    ) -> Option<RayCastResultNormal> {
        let origin = Point3::new(origin.x, origin.y, origin.z);
        let ray = Ray::new(origin, dir.to_na());

        let pipeline = self.query_pipeline.read();
        if let Some((handle, ray_intersection)) = pipeline.cast_ray_and_get_normal(
            &self.rigid_body_set.read(),
            &self.collider_set.read(),
            &ray,
            max_toi,
            true,
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

    /// # Parameters
    /// * `shape` - the shape being cast
    /// * `origin` - the initial position of the shape (this is analog to ray.origin)
    /// * `dir` - the linear velocity the shape is travelling at (this is analog to ray.dir)
    /// * `max_toi` - The maximum time-of-impact that can be reported by this cast. This effectively
    fn cast_shape(
        &self,
        shape: RapierPhysicsShape,
        origin: Vector3,
        dir: Vector3,
        max_toi: f32,
        filter: RapierQueryFilter,
    ) -> Option<ShapeCastResult> {
        let shape_pos = Isometry::new(origin.to_na(), vector![0.0, 0.0, 0.0]);

        let options = ShapeCastOptions::with_max_time_of_impact(max_toi);
        let pipeline = self.query_pipeline.read();
        if let Some((handle, shape_hit)) = pipeline.cast_shape(
            &self.rigid_body_set.read(),
            &self.collider_set.read(),
            &shape_pos,
            &dir.to_na(),
            shape.get_shape(),
            options,
            filter.filter,
        ) {
            let point = origin.to_na() + dir.to_na() * shape_hit.time_of_impact;
            let result = ShapeCastResult {
                collider_id: handle.into_raw_parts().0 as usize,
                point: Vector3::new(point.x, point.y, point.z),
            };
            return Some(result);
        }
        return None;
    }
}
