use godot::{obj::Gd, register::GodotClass};

use super::look_at::LookAt;

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

    pub fn is_main_type(&self) -> bool {
        match self.action_type {
            PlayerActionType::Main => true,
            PlayerActionType::Second => false,
        }
    }

    pub fn _is_second_type(&self) -> bool {
        match self.action_type {
            PlayerActionType::Main => false,
            PlayerActionType::Second => true,
        }
    }

    pub fn get_world_slug(&self) -> &String {
        &self.world_slug
    }
}
