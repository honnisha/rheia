use super::{
    collider::PhysxPhysicsCollider,
    controller::PhysxPhysicsController,
    query_filter::PhysxQueryFilter,
    rigid_body::PhysxPhysicsRigidBody,
    types::{PxRigidStatic, PxShape},
};
use crate::{
    chunks::position::IntoNetworkVector,
    network::messages::Vector3 as NetworkVector3,
    physics::physics::{IPhysicsContainer, RayCastResultNormal},
};
use parking_lot::RwLock;
use physx::owner::Owner;
use physx::{
    math::{PxTransform, PxVec3},
    prelude::{Physics, RigidActor, RigidStatic},
    traits::Class,
};
use physx_sys::{
    phys_PxCreateStatic, PxHitFlags, PxPhysics_createShape_mut, PxSceneQueryExt_raycastSingle, PxScene_addActor_mut,
    PxShapeFlags,
};
use std::{mem::MaybeUninit, sync::Arc};
use std::{ops::Deref, ptr::null_mut};

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
        collider_builder: PhysxPhysicsColliderBuilder,
    ) -> (PhysxPhysicsRigidBody, PhysxPhysicsCollider) {
        let mut controller = self.controller.as_ref().write();

        let mut material = controller.physics.create_material(0.0, 0.0, 0.0, ()).unwrap();
        let geometry = collider_builder.get_geometry(&mut *controller);
        let actor: Owner<PxRigidStatic> = unsafe {
            RigidStatic::from_raw(
                phys_PxCreateStatic(
                    controller.physics.as_mut_ptr(),
                    PxTransform::from_translation(&PxVec3::new(0.0, 0.0, 0.0)).as_ptr(),
                    geometry.deref().as_ptr(),
                    material.as_mut_ptr(),
                    PxTransform::default().as_ptr(),
                ),
                (),
            )
            .unwrap()
        };

        unsafe {
            PxScene_addActor_mut(controller.scene.as_mut_ptr(), actor.as_mut_ptr(), std::ptr::null());
        }
        let rigid_body = PhysxPhysicsRigidBody::create(actor);
    }

    fn spawn_collider(&self, mut collider_builder: PhysxPhysicsColliderBuilder) -> PhysxPhysicsCollider {
        let mut controller = self.controller.as_ref().write();

        let mut actor = controller
            .physics
            .create_static(PxTransform::from_translation(&PxVec3::new(0.0, 0.0, 0.0)), ())
            .unwrap();
        unsafe {
            PxScene_addActor_mut(controller.scene.as_mut_ptr(), actor.as_mut_ptr(), std::ptr::null());
        }

        let mut material = controller.physics.create_material(0.0, 0.0, 0.0, ()).unwrap();
        let geometry = collider_builder.get_geometry(&mut *controller);
        let flags = PxShapeFlags::SceneQueryShape | PxShapeFlags::SimulationShape | PxShapeFlags::Visualization;
        let mut shape: Owner<PxShape> = unsafe {
            physx::shape::Shape::from_raw(
                PxPhysics_createShape_mut(
                    controller.physics.as_mut_ptr(),
                    geometry.deref().as_ptr(),
                    material.as_mut_ptr(),
                    true,
                    flags,
                ),
                (),
            )
            .unwrap()
        };
        actor.attach_shape(&mut shape);
        PhysxPhysicsCollider::create(actor, shape)
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
