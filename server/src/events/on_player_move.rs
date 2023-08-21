use bevy_ecs::{prelude::EventReader, system::ResMut};
use log::info;

use crate::{network::player_container::Players, worlds::worlds_manager::WorldsManager, entities::entity::Position};

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
    mut players: ResMut<Players>,
    mut worlds_manager: ResMut<WorldsManager>,
) {
    for event in player_move_events.iter() {
        let mut player = players.get_mut(&event.client_id);
        info!("Player move \"{}\"", player.get_login());
    }
}
