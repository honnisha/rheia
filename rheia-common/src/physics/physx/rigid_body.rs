use crate::network::messages::Vector3 as NetworkVector3;
use crate::physics::physics::IPhysicsRigidBody;
use physx::owner::Owner;

use super::types::PxRigidStatic;

#[derive(Clone)]
pub struct PhysxPhysicsRigidBody {}

impl PhysxPhysicsRigidBody {
    pub(crate) fn create(actor: Owner<PxRigidStatic>) -> Self {
        Self {}
    }
}

impl IPhysicsRigidBody for PhysxPhysicsRigidBody {
    fn set_enabled(&mut self, _active: bool) {
        todo!()
    }

    fn get_position(&self) -> NetworkVector3 {
        todo!()
    }

    fn set_position(&mut self, _position: NetworkVector3) {
        todo!()
    }
}
