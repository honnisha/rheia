use crate::{entities::entity::NetworkComponent, worlds::worlds_manager::WorldsManager};
use ahash::HashMap;
use bevy_ecs::system::Res;
use common::chunks::chunk_position::ChunkPosition;

use super::{clients_container::ClientRef, server::NetworkContainer};

pub fn send_chunks(worlds_manager: Res<WorldsManager>, network_container: Res<NetworkContainer>) {
    #[cfg(feature = "trace")]
    let _span = bevy_utils::tracing::info_span!("send_chunks").entered();

    // Iterate all worlds
    for (_world_slug, world_lock) in worlds_manager.get_worlds() {
        let world = world_lock.read();

        // A set of chunks and players that require them to be sent
        let mut queue: HashMap<ChunkPosition, Vec<ClientRef>> = Default::default();

        let chunks_map = world.get_chunks_map();

        // Iterate all loaded chunks
        for (chunk_position, chunk_col_lock) in chunks_map.get_chunks() {
            let chunk_col = chunk_col_lock.read();
            if !chunk_col.is_loaded() {
                continue;
            }

            // Get all entites that watch this chunk
            let watch_entities = match chunks_map.get_chunk_watchers(&chunk_position) {
                Some(v) => v,
                None => {
                    panic!("chunk_loaded_event_reader chunk {} not found", chunk_position);
                }
            };
            'entity_loop: for entity in watch_entities {
                let ecs = world.get_ecs();
                let entity_ref = ecs.get_entity(*entity).unwrap();
                let network = entity_ref.get::<NetworkComponent>().unwrap();
                let client = network.get_client();

                let connected = network_container.is_connected(network.get_client_id());
                if !connected {
                    continue 'entity_loop;
                }

                if client.is_queue_limit() {
                    continue 'entity_loop;
                }

                if client.is_already_sended(&chunk_position) {
                    continue 'entity_loop;
                }

                let clients_queue = queue.entry(chunk_position.clone()).or_insert(Default::default());
                client.send_chunk_to_queue(&chunk_position);
                clients_queue.push(client);
            }
        }

        for (chunk_position, clients) in queue {
            let message = world.get_network_chunk_bytes(&chunk_position).unwrap();
            for client in clients.iter() {
                client.send_loaded_chunk(&chunk_position, message.clone());
            }
        }
    }
}
