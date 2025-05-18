use godot::prelude::*;
use physics::physics::RayCastResultNormal;

use crate::world::physics::PhysicsType;

#[derive(Clone, Copy, Debug, PartialEq, GodotClass)]
#[class(no_init)]
pub struct LookAt {
    cast_result: RayCastResultNormal,
    physics_type: PhysicsType,
}

impl LookAt {
    pub fn create(cast_result: RayCastResultNormal, physics_type: PhysicsType) -> Self {
        Self {
            cast_result,
            physics_type,
        }
    }

    pub fn get_cast_result(&self) -> &RayCastResultNormal {
        &self.cast_result
    }

    pub fn get_physics_type(&self) -> &PhysicsType {
        &self.physics_type
    }
}
