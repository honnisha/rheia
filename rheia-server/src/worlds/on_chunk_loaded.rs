use bevy::prelude::EventWriter;
use bevy_ecs::system::Res;

use crate::network::{
    client_network::{ClientNetwork, WorldEntity},
    server::NetworkContainer,
    sync_players::PlayerSpawnEvent,
};

use super::worlds_manager::WorldsManager;

/// Iterates trough all worlds
/// and drain all their loaded chunks
pub fn on_chunk_loaded(
    worlds_manager: Res<WorldsManager>,
    network_container: Res<NetworkContainer>,
    mut player_spawn_events: EventWriter<PlayerSpawnEvent>,
) {
    for (_key, world) in worlds_manager.get_worlds().iter() {
        let w = world.read();
        let loaded_chunks = w.get_chunks_map().drain_loaded_chunks().collect::<Vec<_>>();
        for chunk_position in loaded_chunks {
            let world_slug = w.get_slug().clone();
            let ecs = w.get_ecs();

            'entity_loop: for entity in ecs.get_chunk_entities(&chunk_position).unwrap() {
                let Some(network) = entity.get::<ClientNetwork>() else {
                    continue 'entity_loop;
                };

                let connected = network_container.is_connected(&network);
                if !connected {
                    continue 'entity_loop;
                }

                let world_entity = WorldEntity::new(world_slug.clone(), entity.id());
                player_spawn_events.write(PlayerSpawnEvent::new(world_entity));
            }
        }
    }
}
