use crate::worlds::worlds_manager::WorldsManager;
use ahash::HashMap;
use bevy_ecs::system::Res;
use common::chunks::chunk_position::ChunkPosition;

use super::{client_network::ClientNetwork, server::NetworkContainer};

pub fn send_chunks(worlds_manager: Res<WorldsManager>, network_container: Res<NetworkContainer>) {
    #[cfg(feature = "trace")]
    let _span = bevy_utils::tracing::info_span!("send_chunks").entered();

    // Iterate all worlds
    for (_world_slug, world_lock) in worlds_manager.get_worlds() {
        let world = world_lock.read();

        // A set of chunks and players that require them to be sent
        let mut queue: HashMap<ChunkPosition, Vec<&ClientNetwork>> = Default::default();

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
                let network = entity_ref.get::<ClientNetwork>().unwrap();

                let connected = network_container.is_connected(&network);
                if !connected {
                    continue 'entity_loop;
                }

                if network.is_queue_limit() {
                    continue 'entity_loop;
                }

                if network.is_already_sended(&chunk_position) {
                    continue 'entity_loop;
                }

                let clients_queue = queue.entry(chunk_position.clone()).or_insert(Default::default());
                network.send_chunk_to_queue(&chunk_position);
                clients_queue.push(&network);
            }
        }

        for (chunk_position, clients) in queue {
            let message = world.get_network_chunk_bytes(&chunk_position).unwrap();
            for client in clients.iter() {
                // log::info!("send_loaded_chunk chunk_position:{}", chunk_position);
                client.send_loaded_chunk(&chunk_position, message.clone());
            }
        }
    }
}
