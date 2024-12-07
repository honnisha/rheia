use bevy::prelude::Event;
use bevy_ecs::{
    prelude::EventReader,
    system::{Res, ResMut},
};

use crate::{
    network::{client_network::ClientNetwork, clients_container::ClientsContainer, sync_entities::sync_entity_despawn},
    worlds::worlds_manager::WorldsManager,
};

#[derive(Event)]
pub struct PlayerDisconnectEvent {
    client: ClientNetwork,
    reason: String,
}

impl PlayerDisconnectEvent {
    pub fn new(client: ClientNetwork, reason: String) -> Self {
        Self { client, reason }
    }
}

pub fn on_disconnect(
    mut disconnection_events: EventReader<PlayerDisconnectEvent>,
    mut clients: ResMut<ClientsContainer>,
    worlds_manager: Res<WorldsManager>,
) {
    for event in disconnection_events.read() {
        if let Some(i) = event.client.get_client_info() {
            log::info!(
                target: "network",
                "Disconnected ip:{} login:{} reason:{}",
                event.client.get_client_ip(),
                i.get_login(),
                event.reason
            );
        }

        // Check if player was in the world, despawn if so
        let world_entity = event.client.get_world_entity();
        match world_entity {
            Some(c) => {
                let mut world_manager = worlds_manager.get_world_manager_mut(&c.get_world_slug()).unwrap();
                sync_entity_despawn(&*world_manager, c.get_entity());
                world_manager.despawn_player(&c);
            }
            None => return,
        };
        clients.remove(&event.client.get_client_id());
    }
}
