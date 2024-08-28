use crate::network::messages::{IntoNetworkVector, Vector3 as NetworkVector3};
use crate::physics::physics::IPhysicsRigidBody;
use rapier3d::prelude::*;

use super::bridge::IntoNaVector3;
use super::container::RapierPhysicsContainer;

/// For bodies with physics
#[derive(Clone)]
pub struct RapierPhysicsRigidBody {
    physics_container: RapierPhysicsContainer,
    pub(crate) rigid_handle: RigidBodyHandle,
}

impl RapierPhysicsRigidBody {
    pub(crate) fn create(physics_container: &RapierPhysicsContainer, rigid_handle: RigidBodyHandle) -> Self {
        Self {
            physics_container: physics_container.clone(),
            rigid_handle,
        }
    }
}

impl IPhysicsRigidBody for RapierPhysicsRigidBody {
    fn set_enabled(&mut self, active: bool) {
        let mut body = self
            .physics_container
            .get_rigid_body_mut(&self.rigid_handle)
            .expect("physics entity dosesn't have rigid body");
        body.set_enabled(active);
    }

    fn get_position(&self) -> NetworkVector3 {
        let body = self.physics_container.get_rigid_body(&self.rigid_handle).unwrap();
        body.translation().to_network()
    }

    fn set_position(&mut self, position: NetworkVector3) {
        let mut body = self.physics_container.get_rigid_body_mut(&self.rigid_handle).unwrap();
        body.set_translation(position.to_na(), true);
    }
}
