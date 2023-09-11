use bevy::prelude::Entity;
use common::{
    chunks::chunk_position::ChunkPosition,
    network::{channels::ServerChannel, messages::ServerMessages},
    utils::vec_remove_item,
};
use parking_lot::RwLock;
use core::fmt;
use log::error;
use renet::RenetServer;
use std::{fmt::Display, sync::Arc};

use crate::{
    console::console_sender::{ConsoleSender, ConsoleSenderType},
    entities::entity::{Position, Rotation},
};

use super::server::{NetworkContainer, NetworkPlugin};

static SEND_CHUNK_QUEUE_LIMIT: usize = 128;

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

    // Current chunks that player can see
    // Tp prevent resend chunks
    already_sended: Arc<RwLock<Vec<ChunkPosition>>>,

    // Chunks was sended by network
    // but not yet recieved by the player
    send_chunk_queue: Arc<RwLock<Vec<ChunkPosition>>>,
}

impl ClientNetwork {
    pub fn new(client_id: u64, login: String) -> Self {
        ClientNetwork {
            client_id,
            login,
            world_entity: None,
            already_sended: Default::default(),
            send_chunk_queue: Default::default(),
        }
    }

    pub fn get_login(&self) -> &String {
        &self.login
    }

    pub fn get_client_id(&self) -> &u64 {
        &self.client_id
    }

    pub fn is_connected(&self, server: &RenetServer) -> bool {
        server.clients_id().contains(&self.client_id)
    }

    pub fn send_teleport(&mut self, network_container: &NetworkContainer, position: &Position, rotation: &Rotation) {
        let server = network_container.get_server();
        if !server.clients_id().contains(&self.client_id) {
            error!("send_teleport runs on disconnected user {}", self.login);
            return;
        }

        let world_entity = self.world_entity.as_ref().unwrap();
        let input = ServerMessages::Teleport {
            world_slug: world_entity.get_world_slug().clone(),
            location: position.to_array(),
            yaw: rotation.get_yaw().clone(),
            pitch: rotation.get_pitch().clone(),
        };
        let encoded = bincode::serialize(&input).unwrap();
        NetworkPlugin::send_static_message(self.get_client_id().clone(), ServerChannel::Reliable.into(), encoded);
    }

    pub fn is_already_sended(&self, chunk_position: &ChunkPosition) -> bool {
        self.already_sended.read().contains(chunk_position)
    }

    /// If too many chunks currently was sended and waiting
    /// for confirmation that they have reached the client
    pub fn is_queue_limit(&self) -> bool {
        self.send_chunk_queue.read().len() > SEND_CHUNK_QUEUE_LIMIT
    }

    pub fn send_to_queue(&self, chunk_position: &ChunkPosition) {
        self.send_chunk_queue.write().push(chunk_position.clone());
    }

    /// Called when the player has sent a confirmation of receiving chunk data
    pub fn mark_chunk_as_recieved(&self, chunk_position: ChunkPosition) {
        vec_remove_item(&mut *self.send_chunk_queue.write(), &chunk_position);
    }

    /// Send chunk which was just loaded
    pub fn send_loaded_chunk(&self, chunk_position: &ChunkPosition, encoded: Vec<u8>) {
        if self.already_sended.read().contains(&chunk_position) {
            panic!("Tried to send already sended chunk! {}", chunk_position);
        }
        NetworkPlugin::send_static_message(
            self.get_client_id().clone(),
            ServerChannel::Chunks.into(),
            encoded.clone(),
        );

        // Watch chunk
        self.already_sended.write().push(chunk_position.clone());
    }

    /// Send chunks to unload
    pub fn send_unload_chunks(
        &self,
        network_container: &NetworkContainer,
        world_slug: &String,
        mut abandoned_chunks: Vec<ChunkPosition>,
    ) {
        if abandoned_chunks.len() == 0 {
            return;
        }

        let server = network_container.get_server();
        if !self.is_connected(&*server) {
            error!("send_unload_chunks runs on disconnected user {}", self.login);
            return;
        }

        let mut unload_chunks: Vec<ChunkPosition> = Default::default();

        // Unwatch chunks
        // Send only those chunks, that was sended
        for chunk_position in abandoned_chunks.drain(..) {
            let removed = vec_remove_item(&mut *self.already_sended.write(), &chunk_position);
            if removed {
                unload_chunks.push(chunk_position);
            }
        }
        let input = ServerMessages::UnloadChunks {
            world_slug: world_slug.clone(),
            chunks: unload_chunks,
        };
        let encoded = bincode::serialize(&input).unwrap();
        NetworkPlugin::send_static_message(self.get_client_id().clone(), ServerChannel::Chunks.into(), encoded);
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
