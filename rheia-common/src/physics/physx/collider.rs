use crate::network::messages::Vector3 as NetworkVector3;
use crate::physics::physics::IPhysicsCollider;

#[derive(Clone)]
pub struct PhysxPhysicsCollider {}

impl PhysxPhysicsCollider {
    fn create() -> Self {
        Self {}
    }
}

impl IPhysicsCollider for PhysxPhysicsCollider {
    fn set_enabled(&mut self, _active: bool) {
        todo!();
    }

    fn get_index(&self) -> usize {
        todo!()
    }

    fn set_position(&mut self, _position: NetworkVector3) {
        todo!()
    }

    fn remove(&self) {
        todo!()
    }
}
