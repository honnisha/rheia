use crate::{
    entities::entity::{NetworkComponent, Position},
    worlds::worlds_manager::WorldsManager,
    CHUNKS_DISTANCE,
};
use ahash::HashMap;
use bevy_ecs::{prelude::Entity, system::Res};
use common::chunks::{block_position::BlockPositionTrait, chunk_position::ChunkPosition};
use spiral::ManhattanIterator;

use super::{clients_container::ClientsContainer, server::NetworkContainer};

pub fn send_chunks(
    worlds_manager: Res<WorldsManager>,
    network_container: Res<NetworkContainer>,
    clients: Res<ClientsContainer>,
) {
    let now = std::time::Instant::now();
    let server = network_container.get_server();

    let mut chunks_count: usize = 0;
    let mut entities_count: usize = 0;

    // Iterate all worlds
    for (_world_slug, world_lock) in worlds_manager.get_worlds() {
        let world = world_lock.read();

        // A set of chunks and players that require them to be sent
        let mut queue_chunks: Vec<ChunkPosition> = Default::default();
        let mut queue_entities: Vec<Entity> = Default::default();

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
                let client = clients.get(&network.get_client_id());

                if !client.is_connected(&*server) {
                    continue 'entity_loop;
                }

                if client.is_queue_limit() {
                    continue 'entity_loop;
                }

                if client.is_already_sended(&chunk_position) {
                    continue 'entity_loop;
                }

                if !queue_chunks.contains(&chunk_position) {
                    queue_chunks.push(chunk_position.clone());
                }
                if !queue_entities.contains(&entity) {
                    queue_entities.push(entity.clone());
                }
                client.send_to_queue(&chunk_position);
            }
        }

        chunks_count += queue_chunks.len();
        entities_count += queue_entities.len();

        let mut generated_chunks: HashMap<ChunkPosition, Vec<u8>> = Default::default();
        for chunk_position in queue_chunks.drain(..) {
            let encoded = world.get_network_chunk_bytes(&chunk_position).unwrap();
            generated_chunks.insert(chunk_position, encoded);
        }
        for entity in queue_entities.drain(..) {
            let player_entity = world.get_entity(&entity);
            let network = player_entity.get::<NetworkComponent>().unwrap();
            let client = clients.get(&network.get_client_id());

            let position = player_entity.get::<Position>().unwrap();
            let chunk_position = position.get_chunk_position();

            let iter = ManhattanIterator::new(chunk_position.x as i32, chunk_position.z as i32, CHUNKS_DISTANCE);
            for (x, z) in iter {
                let chunk_position = ChunkPosition::new(x as i64, z as i64);
                if let Some(c) = generated_chunks.get(&chunk_position) {
                    client.send_loaded_chunk(&chunk_position, c.clone());
                }
            }
        }
    }

    let elapsed = now.elapsed();
    if elapsed > std::time::Duration::from_millis(20) {
        println!("send_chunks: {:.2?} chunks_count: {} entities_count: {}", elapsed, chunks_count, entities_count);
    }
}
