use super::player_controller::LookAt;
use godot::{obj::Gd, register::GodotClass};

pub enum PlayerActionType {
    Main,
    Second,
}
/// Used to transmit motion data
#[derive(GodotClass)]
#[class(no_init)]
pub struct PlayerAction {
    hit: Option<Gd<LookAt>>,
    action_type: PlayerActionType,
    world_slug: String,
}

impl PlayerAction {
    pub fn create(hit: Option<Gd<LookAt>>, action_type: PlayerActionType, world_slug: String) -> Self {
        Self {
            hit,
            action_type,
            world_slug,
        }
    }

    pub fn get_hit(&self) -> &Option<Gd<LookAt>> {
        &self.hit
    }

    pub fn get_action_type(&self) -> &PlayerActionType {
        &self.action_type
    }

    pub fn get_world_slug(&self) -> &String {
        &self.world_slug
    }
}
