use bevy_ecs::prelude::EventReader;
use log::info;
use renet::DisconnectReason;

use crate::network::player_network::PlayerNetwork;

pub struct PlayerDisconnectEvent {
    player_network: Box<PlayerNetwork>,
    reason: DisconnectReason,
}

impl PlayerDisconnectEvent {
    pub fn new(player_network: Box<PlayerNetwork>, reason: DisconnectReason) -> Self {
        Self { player_network, reason }
    }
}

pub fn on_disconnect(mut disconnection_events: EventReader<PlayerDisconnectEvent>) {
    for event in disconnection_events.iter() {
        info!(
            "Disconnected login \"{}\" reason {}",
            event.player_network.get_login(),
            event.reason,
        );
    }
}
