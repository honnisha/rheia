use crate::physics::IPhysicsCharacterController;
use common::chunks::position::Vector3;

use super::collider::PhysxPhysicsCollider;
use super::query_filter::PhysxQueryFilter;

pub struct PhysxPhysicsCharacterController {
    custom_mass: Option<f32>,
}

impl IPhysicsCharacterController<PhysxPhysicsCollider, PhysxQueryFilter> for PhysxPhysicsCharacterController {
    fn create(custom_mass: Option<f32>) -> Self {
        Self { custom_mass }
    }

    fn move_shape(
        &mut self,
        _collider: &PhysxPhysicsCollider,
        _filter: PhysxQueryFilter,
        _delta: f64,
        _impulse: Vector3,
    ) -> Vector3 {
        // https://github.com/rlidwka/bevy_mod_physx/blob/ef9e56023fb7500c7e5d1f2b66057a16a3caf8d7/examples/kinematic.rs
        Vector3::zero()
    }

    fn is_grounded(&self) -> bool {
        false
    }

    fn get_custom_mass(&mut self) -> &Option<f32> {
        &self.custom_mass
    }
}
