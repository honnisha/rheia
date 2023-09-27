use bevy::prelude::Event;
use bevy_ecs::prelude::EventReader;
use bevy_ecs::system::{Res, ResMut};
use log::error;

use crate::entities::entity::Rotation;
use crate::network::clients_container::ClientsContainer;
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
    mut player_move_events: EventReader<PlayerMoveEvent>,
    clients: Res<ClientsContainer>,
    worlds_manager: ResMut<WorldsManager>,
) {
    for event in player_move_events.iter() {
        let client = clients.get(&event.client_id);

        let world_entity = client.get_world_entity();
        let world_entity = match world_entity.as_ref() {
            Some(w) => w,
            None => {
                error!(
                    "Client ip:{} tries to send move packets but he not in the world!",
                    client
                );
                continue;
            }
        };

        let mut world_manager = worlds_manager
            .get_world_manager_mut(&world_entity.get_world_slug())
            .unwrap();
        let (chunk_changed, abandoned_chunks) =
            world_manager.player_move(&world_entity, event.position, event.rotation);

        if chunk_changed {
            let world_slug = world_entity.get_world_slug().clone();
            client.send_unload_chunks(&world_slug, abandoned_chunks);
        }
    }
}
