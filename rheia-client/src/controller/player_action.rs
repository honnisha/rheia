use godot::register::GodotClass;
use physics::physics::RayCastResultNormal;

use crate::world::physics::PhysicsType;

pub enum PlayerActionType {
    Main,
    Second,
}
/// Used to transmit motion data
#[derive(GodotClass)]
#[class(no_init)]
pub struct PlayerAction {
    hit: Option<(RayCastResultNormal, PhysicsType)>,
    action_type: PlayerActionType,
}

impl PlayerAction {
    pub fn create(hit: Option<(RayCastResultNormal, PhysicsType)>, action_type: PlayerActionType) -> Self {
        Self { hit, action_type }
    }

    pub fn get_hit(&self) -> &Option<(RayCastResultNormal, PhysicsType)> {
        &self.hit
    }

    pub fn get_action_type(&self) -> &PlayerActionType {
        &self.action_type
    }
}
