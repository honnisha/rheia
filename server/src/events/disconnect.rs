use bevy_ecs::{prelude::EventReader, system::ResMut};
use log::info;
use renet::DisconnectReason;

use crate::network::player_container::Players;

pub struct PlayerDisconnectEvent {
    client_id: u64,
    reason: DisconnectReason,
}

impl PlayerDisconnectEvent {
    pub fn new(client_id: u64, reason: DisconnectReason) -> Self {
        Self { client_id, reason }
    }
}

pub fn on_disconnect(mut disconnection_events: EventReader<PlayerDisconnectEvent>, mut players: ResMut<Players>) {
    for event in disconnection_events.iter() {
        {
            let player = players.get(&event.client_id);
            info!("Disconnected login \"{}\" reason {}", player.get_login(), event.reason,);
        }
        players.remove(&event.client_id);
    }
}
