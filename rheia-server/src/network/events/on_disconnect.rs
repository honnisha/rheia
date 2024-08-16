use bevy::prelude::Event;
use bevy_ecs::{
    prelude::EventReader,
    system::{Res, ResMut},
};

use crate::{network::clients_container::ClientsContainer, worlds::worlds_manager::WorldsManager};

#[derive(Event)]
pub struct PlayerDisconnectEvent {
    client_id: u64,
    reason: String,
}

impl PlayerDisconnectEvent {
    pub fn new(client_id: u64, reason: String) -> Self {
        Self { client_id, reason }
    }
}

pub fn on_disconnect(
    mut disconnection_events: EventReader<PlayerDisconnectEvent>,
    mut clients: ResMut<ClientsContainer>,
    worlds_manager: Res<WorldsManager>,
) {
    for event in disconnection_events.read() {
        {
            let client = clients.get(&event.client_id);
            if let Some(i) = client.get_client_info() {
                log::info!(
                    target: "network",
                    "Disconnected ip:{} login:{} reason:{}",
                    client.get_client_ip(),
                    i.get_login(),
                    event.reason
                );
            }

            // Check if player was in the world, despawn if so
            let world_entity = client.get_world_entity();
            match world_entity {
                Some(c) => {
                    let mut world_manager = worlds_manager.get_world_manager_mut(&c.get_world_slug()).unwrap();
                    world_manager.despawn_player(&c);
                }
                None => return,
            };
        }
        clients.remove(&event.client_id);
    }
}
