use bevy::prelude::Event;
use bevy_ecs::prelude::EventReader;
use bevy_ecs::system::Res;

use crate::entities::entity::Rotation;
use crate::network::clients_container::ClientsContainer;
use crate::network::server::NetworkContainer;
use crate::{entities::entity::Position, worlds::worlds_manager::WorldsManager};

#[derive(Event)]
pub struct PlayerMoveEvent {
    client_id: u64,
    position: Position,
    rotation: Rotation,
}

impl PlayerMoveEvent {
    pub fn new(client_id: u64, position: Position, rotation: Rotation) -> Self {
        Self {
            client_id,
            position,
            rotation,
        }
    }
}

pub fn on_player_move(
    network_container: Res<NetworkContainer>,
    mut player_move_events: EventReader<PlayerMoveEvent>,
    clients: Res<ClientsContainer>,
    worlds_manager: Res<WorldsManager>,
) {
    for event in player_move_events.iter() {
        let mut client = clients.get_mut(&event.client_id);
        if let Some(world_entity) = client.world_entity.as_ref() {
            // Handle player move in world
            let (chunk_changed, abandoned_chunks) = {
                let mut world_manager = worlds_manager
                    .get_world_manager_mut(&world_entity.get_world_slug())
                    .unwrap();
                world_manager.player_move(&world_entity, event.position, event.rotation)
            };

            if chunk_changed {
                let world_slug = world_entity.get_world_slug().clone();
                // Send abandoned chunks to unload
                client.send_unload_chunks(&network_container, &world_slug, abandoned_chunks);

                // Send new chunks
                client.send_already_loaded_chunks(&network_container, &worlds_manager);
            }
        }
    }
}
