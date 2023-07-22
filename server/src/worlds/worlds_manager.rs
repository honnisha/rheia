use super::chunks::chunk_column::LOADED_CHUNKS;
use bevy::prelude::Resource;
use bevy::time::Time;
use bevy_ecs::system::{Res, ResMut};
use common::network::{ServerChannel, ServerMessages};
use dashmap::DashMap;
use log::{error, trace};

use crate::network::{
    player_container::{PlayerMut},
    server::NetworkContainer,
};

use super::world_manager::WorldManager;

/// Contains and manages all worlds of the server
#[derive(Resource)]
pub struct WorldsManager {
    worlds: DashMap<String, WorldManager>,
}

impl Default for WorldsManager {
    fn default() -> Self {
        WorldsManager { worlds: DashMap::new() }
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
        self.worlds.insert(slug.clone(), WorldManager::new(slug, seed));
        Ok(())
    }

    pub fn count(&self) -> usize {
        self.worlds.len()
    }

    pub fn get_worlds(&self) -> &DashMap<String, WorldManager> {
        &self.worlds
    }

    pub fn get_worlds_mut(&mut self) -> &mut DashMap<String, WorldManager> {
        &mut self.worlds
    }

    pub fn get_world_manager(&self, key: &String) -> dashmap::mapref::one::Ref<'_, String, WorldManager> {
        self.worlds.get(key).unwrap()
    }

    pub fn get_world_manager_mut(&self, key: &String) -> dashmap::mapref::one::RefMut<'_, String, WorldManager> {
        self.worlds.get_mut(key).unwrap()
    }

    pub fn spawn_player(&mut self, player_network: &mut PlayerMut, world_slug: &String, x: f32, y: f32, z: f32) {
        let mut world_manager = self.get_world_manager_mut(world_slug);
        world_manager.spawn_player(player_network.get_client_id().clone(), x, y, z);
        player_network.current_world = Some(world_slug.clone());
    }

    pub fn despawn_player(&mut self, player_network: &mut PlayerMut) {
        let current_world = match player_network.current_world.as_ref() {
            Some(c) => c,
            None => return,
        };
        let mut world_manager = self.get_world_manager_mut(&current_world);
        world_manager.despawn_player(player_network.get_client_id());
        player_network.current_world = None;
    }
}

pub fn update_world_chunks(mut worlds_manager: ResMut<WorldsManager>, time: Res<Time>) {
    for mut world in worlds_manager.get_worlds_mut().iter_mut() {
        world.update_chunks(time.delta());
    }
}

pub fn chunk_loaded_event_reader(
    worlds_manager: ResMut<WorldsManager>,
    network_container: Res<NetworkContainer>,
) {
    let mut server = network_container.server.write().expect("poisoned");

    // Iterate loaded chunks
    for (world_slug, chunk_position) in LOADED_CHUNKS.1.drain() {
        let world = worlds_manager.get_world_manager(&world_slug);

        // Get all clients which is waching this chunk
        let watch_clients = match world.chunks.take_chunks_clients(&chunk_position) {
            Some(v) => v,
            None => {
                panic!("chunk_loaded_event_reader chunk {} not found", chunk_position);
            },
        };

        if watch_clients.len() <= 0 {
            continue;
        }

        // Try to get chunk data
        let encoded = match world.chunks.get_chunk_column(&chunk_position) {
            Some(chunk_column) => {
                let input = ServerMessages::ChunkSectionInfo {
                    sections: chunk_column.build_network_format(),
                    chunk_position: chunk_position.clone(),
                };
                bincode::serialize(&input).unwrap()
            },
            None => {
                error!(
                    "chunk_loaded_event_reader there is not chunk for player_chunks_watch:{}",
                    chunk_position
                );
                continue;
            },
        };

        for client_id in watch_clients {
            server.send_message(client_id.clone(), ServerChannel::Messages, encoded.clone());
            trace!(target: "network", "NETWORK Send chunk {} to the client {}", chunk_position, client_id);
        }
    }
}
