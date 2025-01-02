use bevy::prelude::Event;
use bevy_ecs::prelude::EventReader;
use bevy_ecs::system::ResMut;
use common::chunks::block_position::BlockPositionTrait;

use crate::entities::entity::Rotation;
use crate::network::client_network::{ClientNetwork, WorldEntity};
use crate::network::sync_players::sync_player_move;
use crate::worlds::world_manager::WorldManager;
use crate::{entities::entity::Position, worlds::worlds_manager::WorldsManager};

#[derive(Event)]
pub struct PlayerMoveEvent {
    client: ClientNetwork,
    position: Position,
    rotation: Rotation,
}

impl PlayerMoveEvent {
    pub fn new(client: ClientNetwork, position: Position, rotation: Rotation) -> Self {
        Self {
            client,
            position,
            rotation,
        }
    }
}

pub fn on_player_move(mut player_move_events: EventReader<PlayerMoveEvent>, worlds_manager: ResMut<WorldsManager>) {
    for event in player_move_events.read() {
        let world_entity = event.client.get_world_entity();
        let world_entity = match world_entity.as_ref() {
            Some(w) => w,
            None => {
                log::error!(
                    target: "network",
                    "Client ip:{} tries to send move packets but he is not in the world!",
                    event.client.get_client_ip()
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
                event.client.get_client_ip(), event.position.get_chunk_position()
            );
            continue;
        }
        move_player(&mut *world_manager, world_entity, event.position, event.rotation);
    }
}

/// Move player inside the world
pub fn move_player(
    world_manager: &mut WorldManager,
    world_entity: &WorldEntity,
    position: Position,
    rotation: Rotation,
) {
    let chunks_changed = world_manager.player_move(&world_entity, position, rotation);

    if let Some(change) = chunks_changed.as_ref() {
        let ecs = world_manager.get_ecs();
        let entity_ref = ecs.get_entity(world_entity.get_entity()).unwrap();

        let network = entity_ref.get::<ClientNetwork>().unwrap();
        network.send_unload_chunks(world_entity.get_world_slug(), change.abandoned_chunks.clone());
    }

    sync_player_move(world_manager, world_entity.get_entity(), &chunks_changed);
}
