use bevy::prelude::Event;
use bevy_ecs::prelude::EventReader;
use bevy_ecs::system::{Res, ResMut};
use common::chunks::block_position::BlockPositionTrait;

use crate::entities::entity::Rotation;
use crate::network::client_network::{ClientNetwork, WorldEntity};
use crate::network::clients_container::ClientsContainer;
use crate::network::sync_entities::sync_player_move;
use crate::worlds::world_manager::WorldManager;
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
    for event in player_move_events.read() {
        let client = clients.get(&event.client_id);

        let world_entity = client.get_world_entity();
        let world_entity = match world_entity.as_ref() {
            Some(w) => w,
            None => {
                log::error!(
                    target: "network",
                    "Client ip:{} tries to send move packets but he is not in the world!",
                    client
                );
                continue;
            }
        };

        let mut world_manager = worlds_manager
            .get_world_manager_mut(&world_entity.get_world_slug())
            .unwrap();

        if !world_manager
            .get_chunks_map()
            .is_chunk_loaded(&event.position.get_chunk_position())
        {
            log::warn!(
                target: "network",
                "Client ip:{} tries to move inside loading chunk {}",
                client, event.position.get_chunk_position()
            );
            continue;
        }
        move_player(
            &mut *world_manager,
            &*clients,
            &client,
            world_entity,
            event.position,
            event.rotation,
        );
    }
}

/// Move player inside the world
pub fn move_player(
    world_manager: &mut WorldManager,
    clients: &ClientsContainer,
    client: &ClientNetwork,
    world_entity: &WorldEntity,
    position: Position,
    rotation: Rotation,
) {
    let chunks_changed = world_manager.player_move(&world_entity, position, rotation);

    if let Some((abandoned_chunks, _new_chunks)) = chunks_changed.as_ref() {
        client.send_unload_chunks(world_entity.get_world_slug(), abandoned_chunks.clone());
    }

    sync_player_move(world_manager, world_entity.clone(), clients, &chunks_changed);
}
