use core::fmt;
use std::fmt::Display;

use bevy::prelude::Entity;
use common::network::{channels::ServerChannel, messages::ServerMessages};

use crate::{
    console::console_sender::{ConsoleSender, ConsoleSenderType},
    entities::entity::{Position, Rotation},
    worlds::worlds_manager::WorldsManager,
};

use super::server::{NetworkContainer, NetworkPlugin};

/// Store player current world slug and his entity
#[derive(Clone)]
pub struct WorldEntity {
    current_world_slug: String,
    entity: Entity,
}

impl WorldEntity {
    pub fn new(current_world_slug: String, entity: Entity) -> Self {
        Self {
            current_world_slug,
            entity,
        }
    }

    pub fn get_entity(&self) -> Entity {
        self.entity
    }

    pub fn get_world_slug(&self) -> &String {
        &self.current_world_slug
    }
}

#[derive(Clone)]
pub struct ClientNetwork {
    client_id: u64,
    login: String,

    // For fast finding player current world slug
    pub world_entity: Option<WorldEntity>,
}

impl ClientNetwork {
    pub fn new(client_id: u64, login: String) -> Self {
        ClientNetwork {
            client_id,
            login,
            world_entity: None,
        }
    }

    pub fn get_login(&self) -> &String {
        &self.login
    }

    pub fn get_client_id(&self) -> &u64 {
        &self.client_id
    }

    pub fn send_teleport(&mut self, network_container: &NetworkContainer, position: &Position, rotation: &Rotation) {
        let mut server = network_container.server.write().expect("poisoned");
        let world_entity = self.world_entity.as_ref().unwrap();

        let input = ServerMessages::Teleport {
            world_slug: world_entity.get_world_slug().clone(),
            location: position.to_array(),
            yaw: rotation.get_yaw().clone(),
            pitch: rotation.get_pitch().clone(),
        };
        let encoded = bincode::serialize(&input).unwrap();
        server.send_message(self.client_id.clone(), ServerChannel::Reliable, encoded)
    }

    /// Send already loaded chunks to the client
    pub fn send_loaded_chunks(&self, network_container: &NetworkContainer, worlds_manager: &WorldsManager) {
        let mut server = network_container.server.write().expect("poisoned");
        let world_entity = self.world_entity.as_ref().unwrap();

        let world_manager = worlds_manager.get_world_manager(&world_entity.get_world_slug()).unwrap();
        let client_chunks = world_manager
            .chunks_map
            .take_entity_chunks(&world_entity.get_entity())
            .unwrap();
        for chunk_position in client_chunks {
            if let Some(e) = world_manager.get_network_chunk_bytes(chunk_position) {
                server.send_message(self.get_client_id().clone(), ServerChannel::Reliable, e);
            };
        }
    }
}

impl Display for ClientNetwork {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.login)
    }
}

impl ConsoleSender for ClientNetwork {
    fn send_console_message(&self, message: String) {
        NetworkPlugin::send_console_output(self.client_id.clone(), message);
    }
}
impl ConsoleSenderType for ClientNetwork {}
