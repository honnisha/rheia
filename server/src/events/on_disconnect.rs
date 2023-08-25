use bevy::prelude::Event;
use bevy_ecs::{prelude::EventReader, system::ResMut};
use log::info;
use renet::DisconnectReason;

use crate::{network::clients_container::ClientsContainer, worlds::worlds_manager::WorldsManager};

#[derive(Event)]
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
    mut clients: ResMut<ClientsContainer>,
    mut worlds_manager: ResMut<WorldsManager>,
) {
    for event in disconnection_events.iter() {
        {
            let mut client = clients.get_mut(&event.client_id);
            info!("Disconnected login \"{}\" reason {}", client.get_login(), event.reason,);

            worlds_manager.despawn_player(&mut client)
        }
        clients.remove(&event.client_id);
    }
}
