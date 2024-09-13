use super::controller::PhysxPhysicsController;
use crate::physics::physics::IPhysicsCollider;
use crate::{chunks::position::IntoNetworkVector, network::messages::Vector3 as NetworkVector3};
use parking_lot::RwLock;
use physx::actor::Actor;
use physx::math::PxTransform;
use physx::shape::Shape;
use physx::traits::Class;
use physx::{owner::Owner, prelude::RigidActor};
use physx_sys::PxScene_removeActor_mut;
use std::sync::Arc;

use super::bridge::IntoPxVec3;
use super::types::{PxRigidStatic, PxShape};

pub struct PhysxPhysicsCollider {
    controller: Arc<RwLock<PhysxPhysicsController>>,
    pub(crate) actor: Owner<PxRigidStatic>,
    shape: Owner<PxShape>,
}

impl PhysxPhysicsCollider {
    pub(crate) fn create(
        controller: Arc<RwLock<PhysxPhysicsController>>,
        actor: Owner<PxRigidStatic>,
        shape: Owner<PxShape>,
    ) -> Self {
        Self {
            controller,
            actor,
            shape,
        }
    }
}

impl IPhysicsCollider for PhysxPhysicsCollider {
    fn get_position(&self) -> NetworkVector3 {
        self.actor.get_global_position().to_network()
    }

    fn set_enabled(&mut self, active: bool) {
        self.actor.enable_gravity(active);
    }

    fn get_index(&self) -> usize {
        self.shape.get_user_data().clone()
    }

    fn set_position(&mut self, position: NetworkVector3) {
        self.actor
            .set_global_pose(&PxTransform::from_translation(&position.to_physx()), true);
    }

    fn remove(&mut self) {
        let mut controller = self.controller.as_ref().write();
        unsafe {
            PxScene_removeActor_mut(controller.scene.as_mut_ptr(), self.actor.as_mut_ptr(), false);
        }
    }
}
