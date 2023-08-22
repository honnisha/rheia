use bevy_ecs::prelude::EventReader;
use bevy_ecs::system::Res;

use crate::entities::entity::Rotation;
use crate::network::clients_container::ClientsContainer;
use crate::network::server::NetworkContainer;
use crate::{entities::entity::Position, worlds::worlds_manager::WorldsManager};

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
        let client = clients.get(&event.client_id);
        if let Some(world_entity) = client.world_entity.as_ref() {
            let mut world_manager = worlds_manager
                .get_world_manager_mut(&world_entity.get_world_slug())
                .unwrap();
            let abandoned_chunks = world_manager.player_move(&world_entity, event.position, event.rotation);
            client.send_unload_chunks(&network_container, abandoned_chunks);
        }
    }
}
