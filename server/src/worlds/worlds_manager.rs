use std::sync::Arc;

use super::chunks::chunk_column::LOADED_CHUNKS;
use ahash::HashMap;
use bevy::prelude::Resource;
use bevy::time::Time;
use bevy_ecs::system::{Res, ResMut};
use log::{error, trace};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::{
    entities::entity::NetworkComponent,
    network::{
        clients_container::{ClientMut, ClientsContainer},
        server::NetworkContainer,
    },
};

use super::world_manager::WorldManager;

type WorldsType = HashMap<String, Arc<RwLock<WorldManager>>>;

/// Contains and manages all worlds of the server
#[derive(Resource)]
pub struct WorldsManager {
    worlds: WorldsType,
}

impl Default for WorldsManager {
    fn default() -> Self {
        WorldsManager {
            worlds: Default::default(),
        }
    }
}

impl WorldsManager {
    pub fn has_world_with_slug(&self, slug: &String) -> bool {
        self.worlds.contains_key(slug)
    }

    pub fn create_world(&mut self, slug: String, seed: u64) -> Result<(), String> {
        if self.worlds.contains_key(&slug) {
            return Err(format!("World with slug \"{}\" already exists", slug));
        }
        self.worlds
            .insert(slug.clone(), Arc::new(RwLock::new(WorldManager::new(slug, seed))));
        Ok(())
    }

    pub fn count(&self) -> usize {
        self.worlds.len()
    }

    pub fn get_worlds(&self) -> &WorldsType {
        &self.worlds
    }

    pub fn get_world_manager(&self, key: &String) -> Option<RwLockReadGuard<WorldManager>> {
        match self.worlds.get(key) {
            Some(w) => Some(w.read()),
            None => None,
        }
    }

    pub fn get_world_manager_mut(&self, key: &String) -> Option<RwLockWriteGuard<WorldManager>> {
        match self.worlds.get(key) {
            Some(w) => Some(w.write()),
            None => return None,
        }
    }

    pub fn despawn_player(&mut self, client: &mut ClientMut) {
        let world_entity = match client.world_entity.as_ref() {
            Some(c) => c,
            None => return,
        };
        let mut world_manager = self.get_world_manager_mut(&world_entity.get_world_slug()).unwrap();
        world_manager.despawn_player(&world_entity);
        client.world_entity = None;
    }
}

pub fn update_world_chunks(worlds_manager: Res<WorldsManager>, time: Res<Time>) {
    for (_key, world) in worlds_manager.get_worlds().iter() {
        world.write().update_chunks(time.delta());
    }
}

pub fn chunk_loaded_event_reader(
    worlds_manager: ResMut<WorldsManager>,
    network_container: Res<NetworkContainer>,
    clients: Res<ClientsContainer>,
) {
    let mut server = network_container.get_server_mut();

    // Iterate loaded chunks
    for (world_slug, chunk_position) in LOADED_CHUNKS.1.drain() {
        let world = worlds_manager.get_world_manager(&world_slug).unwrap();

        // Get all clients which is waching this chunk
        let watch_entities = match world.chunks_map.take_chunks_entities(&chunk_position) {
            Some(v) => v,
            None => {
                panic!("chunk_loaded_event_reader chunk {} not found", chunk_position);
            }
        };

        if watch_entities.len() <= 0 {
            continue;
        }

        // Try to get chunk data
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

        match bincode::serialized_size(&encoded) {
            Ok(s) => trace!("NETWORK chunk_position:{} packet size:{}", chunk_position, s),
            Err(e) => error!("NETWORK bincode::serialized_size error: {}", e),
        }
        for entity in watch_entities {
            let player_entity = world.get_entity(entity);
            let network = player_entity.get::<NetworkComponent>().unwrap();
            let mut client = clients.get_mut(&network.get_client_id());
            client.send_loaded_chunk(&mut server, &chunk_position, encoded.clone());
        }
    }
}
