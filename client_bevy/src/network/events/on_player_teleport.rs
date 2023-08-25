use std::collections::HashMap;

use bevy::prelude::{Event, EventReader, Transform, ResMut};

use crate::world::worlds_manager::WorldsManager;

#[derive(Event)]
pub struct PlayerTeleportEvent {
    world_slug: String,
    transform: Transform,
    yaw: f32,
    pitch: f32,
}

impl PlayerTeleportEvent {
    pub fn new(world_slug: String, transform: Transform, yaw: f32, pitch: f32) -> Self {
        Self {
            world_slug,
            transform,
            yaw,
            pitch,
        }
    }
}

pub fn on_player_teleport(
    mut player_teleport_event: EventReader<PlayerTeleportEvent>,
    mut worlds_manager: ResMut<WorldsManager>,
) {
    for event in player_teleport_event.iter() {
        match worlds_manager.get_world_slug() {
            Some(slug) => {
                // Teleport to new world
                if *slug != event.world_slug {
                    worlds_manager.unload_world();
                    worlds_manager.load_world(event.world_slug.clone());
                }
            },
            None => {
                worlds_manager.load_world(event.world_slug.clone());
            },
        }
    }
}
