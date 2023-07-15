use bevy_ecs::{prelude::EventReader, system::ResMut};
use log::info;
use renet::DisconnectReason;

use crate::{network::player_container::Players, worlds::worlds_manager::WorldsManager};

pub struct PlayerDisconnectEvent {
    client_id: u64,
    reason: DisconnectReason,
}

impl PlayerDisconnectEvent {
    pub fn new(client_id: u64, reason: DisconnectReason) -> Self {
        Self { client_id, reason }
    }
}

pub fn on_disconnect(
    mut disconnection_events: EventReader<PlayerDisconnectEvent>,
    mut players: ResMut<Players>,
    mut worlds_manager: ResMut<WorldsManager>,
) {
    for event in disconnection_events.iter() {
        {
            let mut player = players.get_mut(&event.client_id);
            info!("Disconnected login \"{}\" reason {}", player.get_login(), event.reason,);

            worlds_manager.despawn_player(&mut player)
        }
        players.remove(&event.client_id);
    }
}
