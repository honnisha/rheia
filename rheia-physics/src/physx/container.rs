use super::{
    bridge::{network_to_physx_sys, physx_sys_to_network}, collider::PhysxPhysicsCollider, controller::PhysxPhysicsController,
    query_filter::PhysxQueryFilter, types::PxShape,
};
use crate::physics::{IPhysicsContainer, RayCastResultNormal};
use common::chunks::position::Vector3;
use parking_lot::RwLock;
use physx::{
    math::{PxTransform, PxVec3},
    prelude::{Physics, RigidActor},
    traits::Class,
};
use physx::{owner::Owner, shape::Shape};
use physx_sys::{
    PxHitFlags, PxPhysics_createShape_mut, PxSceneQueryExt_raycastSingle, PxScene_addActor_mut, PxShapeFlags,
};
use std::{mem::MaybeUninit, sync::Arc};
use std::{ops::Deref, ptr::null_mut};

use super::collider_builder::PhysxPhysicsColliderBuilder;

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

impl IPhysicsContainer<PhysxPhysicsCollider, PhysxPhysicsColliderBuilder, PhysxQueryFilter> for PhysxPhysicsContainer {
    fn step(&self, delta: f32) {
        self.controller.as_ref().write().step(delta, self);
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
        //panic!("geometry.deref().as_ptr():{:?}", geometry.deref().as_ptr());
        let mut shape: Owner<PxShape> = unsafe {
            physx::shape::Shape::from_raw(
                PxPhysics_createShape_mut(
                    controller.physics.as_mut_ptr(),
                    geometry.deref().as_ptr(),
                    material.as_mut_ptr(),
                    true,
                    flags,
                ),
                rand::random::<usize>(),
            )
            .unwrap()
        };
        actor.attach_shape(&mut shape);
        PhysxPhysicsCollider::create(self.controller.clone(), actor, shape)
    }

    fn cast_ray_and_get_normal(
        &self,
        dir: Vector3,
        max_toi: f32,
        origin: Vector3,
        filter: PhysxQueryFilter,
    ) -> Option<RayCastResultNormal> {
        let controller = self.controller.as_ref().read();

        let mut raycast_hit = MaybeUninit::uninit();

        if !unsafe {
            PxSceneQueryExt_raycastSingle(
                controller.scene.as_ptr(),
                &network_to_physx_sys(&origin),
                &network_to_physx_sys(&dir),
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
            collider_id: unsafe { get_shape_id_from_ptr(raycast_hit.shape) },
            point: physx_sys_to_network(&raycast_hit.position),
            normal: physx_sys_to_network(&raycast_hit.normal),
        })
    }
}

unsafe fn get_shape_id_from_ptr(shape: *const physx_sys::PxShape) -> usize {
    let shape = &*(shape as *const PxShape);
    *shape.get_user_data()
}

#[cfg(test)]
mod tests {
    use crate::{physics::IPhysicsContainer, physx::container::PhysxPhysicsContainer};

    #[test]
    fn test_create_and_step() {
        let container = PhysxPhysicsContainer::default();
        container.step(1.0);
    }
}
