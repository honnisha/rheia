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
    world_slug: String,
}

impl PlayerAction {
    pub fn create(
        hit: Option<(RayCastResultNormal, PhysicsType)>,
        action_type: PlayerActionType,
        world_slug: String,
    ) -> Self {
        Self {
            hit,
            action_type,
            world_slug,
        }
    }

    pub fn get_hit(&self) -> &Option<(RayCastResultNormal, PhysicsType)> {
        &self.hit
    }

    pub fn get_action_type(&self) -> &PlayerActionType {
        &self.action_type
    }

    pub fn get_world_slug(&self) -> &String {
        &self.world_slug
    }
}
