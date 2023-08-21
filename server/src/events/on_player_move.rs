use bevy_ecs::prelude::EventReader;
use bevy_ecs::system::{Res, ResMut};

use crate::{entities::entity::Position, network::player_container::Players, worlds::worlds_manager::WorldsManager};

pub struct PlayerMoveEvent {
    client_id: u64,
    position: Position,
    yaw: f32,
    pitch: f32,
}

impl PlayerMoveEvent {
    pub fn new(client_id: u64, position: Position, yaw: f32, pitch: f32) -> Self {
        Self {
            client_id,
            position,
            yaw,
            pitch,
        }
    }
}

pub fn on_player_move(
    mut player_move_events: EventReader<PlayerMoveEvent>,
    players: Res<Players>,
    mut worlds_manager: ResMut<WorldsManager>,
) {
    for event in player_move_events.iter() {
        let player = players.get(&event.client_id);
        if let Some(world_slug) = player.current_world.as_ref() {
            let mut world_manager = worlds_manager.get_world_manager_mut(&world_slug);
            world_manager.move_player(
                &player,
                position,
                yaw,
                pitch,
            )
        }
    }
}
