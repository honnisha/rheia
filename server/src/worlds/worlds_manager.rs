use super::chunks::chunk_column::LOADED_CHUNKS;
use bevy::prelude::Resource;
use bevy::time::Time;
use bevy_ecs::system::{Res, ResMut};
use common::network::{ServerChannel, ServerMessages};
use dashmap::DashMap;
use log::{error, info, trace};

use crate::network::{
    player_container::{PlayerMut, Players},
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

    pub fn _get_world_manager(&self, key: &String) -> dashmap::mapref::one::Ref<'_, String, WorldManager> {
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
    for (world_slug, chunk_position) in LOADED_CHUNKS.1.try_iter() {
        let mut world = worlds_manager.get_world_manager_mut(&world_slug);

        // Get all clients which is waching this chunk
        let watch_clients = world.chunks.chunks_load_state.take_chunks_clients(&chunk_position);

        for client_id in watch_clients {
            // Try to get chunk data
            if let Some(c) = world.chunks.chunks.get(&chunk_position) {
                trace!(target: "network", "Send chunk {} to the client {}", chunk_position, client_id);

                let input = ServerMessages::ChunkSectionInfo {
                    sections: c.sections.clone(),
                    chunk_position: [chunk_position.x.clone(), chunk_position.z.clone()],
                };
                let encoded = bincode::serialize(&input).unwrap();
                server.send_message(client_id, ServerChannel::Messages, encoded);
            } else {
                error!(
                    "chunk_loaded_event_reader there is not chunk for player_chunks_watch:{}",
                    chunk_position
                );
            }
        }
    }
}
