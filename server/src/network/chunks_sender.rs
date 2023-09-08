use crate::{entities::entity::NetworkComponent, worlds::worlds_manager::WorldsManager};
use ahash::HashMap;
use bevy_ecs::{
    prelude::Entity,
    system::{Res, ResMut},
};
use common::chunks::chunk_position::ChunkPosition;
use log::error;

use super::{clients_container::ClientsContainer, server::NetworkContainer};

pub fn send_chunks(
    worlds_manager: ResMut<WorldsManager>,
    network_container: Res<NetworkContainer>,
    clients: Res<ClientsContainer>,
) {
    let mut server = network_container.get_server_mut();

    // Iterate all worlds
    for (_world_slug, world_lock) in worlds_manager.get_worlds() {
        let world = world_lock.read();

        // A set of chunks and players that require them to be sent
        let mut queue: HashMap<ChunkPosition, Vec<Entity>> = Default::default();

        // Iterate all loaded chunks
        for (chunk_position, chunk_col_lock) in world.chunks_map.get_chunks() {
            let chunk_col = chunk_col_lock.read();
            if !chunk_col.is_loaded() {
                continue;
            }

            // Get all entites that watch this chunk
            let watch_entities = match world.chunks_map.take_chunks_entities(&chunk_position) {
                Some(v) => v,
                None => {
                    panic!("chunk_loaded_event_reader chunk {} not found", chunk_position);
                }
            };
            'entity_loop: for entity in watch_entities {
                let player_entity = world.get_entity(entity);
                let network = player_entity.get::<NetworkComponent>().unwrap();
                let mut client = clients.get_mut(&network.get_client_id());

                if !client.is_connected(&*server) {
                    continue 'entity_loop;
                }

                if client.is_queue_limit() {
                    continue 'entity_loop;
                }

                if client.is_already_sended(&chunk_position) {
                    continue 'entity_loop;
                }

                let chunk_queue = queue.entry(chunk_position.clone()).or_insert(Default::default());
                chunk_queue.push(entity.clone());
                client.send_to_queue(&chunk_position);
            }
        }

        // Sending chunks to players
        for (chunk_position, entities) in queue {
            let encoded = match world.get_network_chunk_bytes(&chunk_position) {
                Some(e) => e,
                None => {
                    error!(
                        "chunk_loaded_event_reader there is not chunk for player_chunks_watch:{}",
                        chunk_position
                    );
                    continue;
                }
            };

            for entity in entities.iter() {
                println!("sended {}", chunk_position);
                let player_entity = world.get_entity(&entity);
                let network = player_entity.get::<NetworkComponent>().unwrap();
                let mut client = clients.get_mut(&network.get_client_id());
                client.send_loaded_chunk(&mut server, &chunk_position, encoded.clone());
            }
        }
    }
}
