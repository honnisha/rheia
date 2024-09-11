use super::{
    collider::PhysxPhysicsCollider, controller::PhysxPhysicsController, query_filter::PhysxQueryFilter,
    rigid_body::PhysxPhysicsRigidBody,
};
use crate::{
    chunks::position::IntoNetworkVector,
    network::messages::Vector3 as NetworkVector3,
    physics::physics::{IPhysicsContainer, RayCastResultNormal},
};
use parking_lot::RwLock;
use physx::traits::Class;
use physx_sys::{PxHitFlags, PxSceneQueryExt_raycastSingle};
use std::ptr::null_mut;
use std::{mem::MaybeUninit, sync::Arc};

use super::{bridge::IntoPxVec3, collider_builder::PhysxPhysicsColliderBuilder};

#[derive(Clone)]
pub struct PhysxPhysicsContainer {
    controller: Arc<RwLock<PhysxPhysicsController>>,
}

impl Default for PhysxPhysicsContainer {
    fn default() -> Self {
        let controller = Arc::new(RwLock::new(PhysxPhysicsController::create()));
        Self { controller }
    }
}

impl IPhysicsContainer<PhysxPhysicsRigidBody, PhysxPhysicsCollider, PhysxPhysicsColliderBuilder, PhysxQueryFilter>
    for PhysxPhysicsContainer
{
    fn step(&self, delta: f32) {
        self.controller.as_ref().write().step(delta, self);
    }

    fn spawn_rigid_body(
        &self,
        _collider_builder: PhysxPhysicsColliderBuilder,
    ) -> (PhysxPhysicsRigidBody, PhysxPhysicsCollider) {
        todo!()
    }

    fn spawn_collider(&self, _collider_builder: PhysxPhysicsColliderBuilder) -> PhysxPhysicsCollider {
        todo!()
    }

    fn cast_ray_and_get_normal(
        &self,
        dir: NetworkVector3,
        max_toi: f32,
        origin: NetworkVector3,
        filter: PhysxQueryFilter,
    ) -> Option<RayCastResultNormal> {
        let controller = self.controller.as_ref().read();

        let mut raycast_hit = MaybeUninit::uninit();

        if !unsafe {
            PxSceneQueryExt_raycastSingle(
                controller.scene.as_ptr(),
                &origin.to_physx_sys(),
                &dir.to_physx_sys(),
                max_toi,
                PxHitFlags::Default,
                raycast_hit.as_mut_ptr(),
                &filter.filter as *const _,
                null_mut(),
                null_mut(),
            )
        } {
            return None;
        }

        // SAFETY: raycastSingle returned true, so we assume buffer is initialized
        let raycast_hit = unsafe { raycast_hit.assume_init() };

        Some(RayCastResultNormal {
            collider_id: todo!(),
            point: raycast_hit.position.to_network(),
            normal: raycast_hit.normal.to_network(),
        })
    }
}
