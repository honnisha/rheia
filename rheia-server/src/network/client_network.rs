use bevy::prelude::Entity;
use common::{chunks::chunk_position::ChunkPosition, utils::vec_remove_item};
use core::fmt;
use flume::{Drain, Receiver, Sender};
use network::messages::{NetworkMessageType, ServerMessages};
use parking_lot::RwLock;
use std::{any::Any, fmt::Display, sync::Arc};

use crate::{
    console::console_sender::{ConsoleSender, ConsoleSenderType},
    entities::entity::{Position, Rotation},
};

use super::{events::on_connection_info::PlayerConnectionInfoEvent, server::{NetworkPlugin, SendClientMessageEvent}};

static SEND_CHUNK_QUEUE_LIMIT: usize = 64;

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
pub struct ClientInfo {
    login: String,
    version: String,
    architecture: String,
    _rendering_device: String,
}

impl ClientInfo {
    pub fn new(event: &PlayerConnectionInfoEvent) -> Self {
        Self {
            login: event.login.clone(),
            version: event.version.clone(),
            architecture: event.architecture.clone(),
            _rendering_device: event.rendering_device.clone(),
        }
    }

    pub fn get_login(&self) -> &String {
        &self.login
    }

    pub fn get_version(&self) -> &String {
        &self.version
    }

    pub fn get_architecture(&self) -> &String {
        &self.architecture
    }
}

#[derive(Clone)]
pub struct ClientNetwork {
    client_id: u64,
    ip: String,

    client_info: Option<ClientInfo>,

    // For fast finding player current world slug
    world_entity: Arc<RwLock<Option<WorldEntity>>>,

    // Current chunks that player can see
    // Tp prevent resend chunks
    already_sended: Arc<RwLock<Vec<ChunkPosition>>>,

    // Chunks was sended by network
    // but not yet recieved by the player
    send_chunk_queue: Arc<RwLock<Vec<ChunkPosition>>>,

    client_messages_output: (Sender<SendClientMessageEvent>, Receiver<SendClientMessageEvent>),
}

impl ClientNetwork {
    pub fn new(client_id: u64, ip: String) -> Self {
        ClientNetwork {
            client_id,
            ip,
            client_info: None,
            world_entity: Arc::new(RwLock::new(None)),
            already_sended: Default::default(),
            send_chunk_queue: Default::default(),
            client_messages_output: flume::unbounded(),
        }
    }

    pub fn get_client_id(&self) -> u64 {
        self.client_id
    }

    pub fn send_allow_connection(&self) {
        self.send_message(NetworkMessageType::ReliableOrdered, ServerMessages::AllowConnection {});
    }

    pub fn get_client_info(&self) -> Option<&ClientInfo> {
        match self.client_info.as_ref() {
            Some(i) => Some(&i),
            None => None,
        }
    }

    pub fn set_client_info(&mut self, info: ClientInfo) {
        self.client_info = Some(info);
    }

    pub fn get_client_ip(&self) -> &String {
        &self.ip
    }

    /// Stores the player's information about which world he is in
    pub fn get_world_entity(&self) -> Option<WorldEntity> {
        let lock = self.world_entity.as_ref().read();
        lock.clone()
    }

    pub fn set_world_entity(&self, world_entity: Option<WorldEntity>) {
        *self.world_entity.write() = world_entity;
    }

    /// Sends information about the player's new position over the network
    pub fn network_send_teleport(&self, position: &Position, rotation: &Rotation) {
        let lock = self.get_world_entity();
        let world_entity = lock.as_ref().unwrap();
        let input = ServerMessages::Teleport {
            world_slug: world_entity.get_world_slug().clone(),
            position: position.to_network(),
            rotation: rotation.to_network(),
        };
        self.send_message(NetworkMessageType::ReliableOrdered, input);
    }

    pub fn is_already_sended(&self, chunk_position: &ChunkPosition) -> bool {
        self.already_sended.read().contains(chunk_position) || self.send_chunk_queue.read().contains(chunk_position)
    }

    /// If too many chunks currently was sended and waiting
    /// for confirmation that they have reached the client
    pub fn is_queue_limit(&self) -> bool {
        self.send_chunk_queue.read().len() > SEND_CHUNK_QUEUE_LIMIT
    }

    pub fn send_chunk_to_queue(&self, chunk_position: &ChunkPosition) {
        self.send_chunk_queue.write().push(chunk_position.clone());
    }

    /// Called when the player has sent a confirmation of receiving chunk data
    pub fn mark_chunks_as_recieved(&self, chunk_positions: Vec<ChunkPosition>) {
        let mut send_chunk_queue = self.send_chunk_queue.write();
        for chunk_position in chunk_positions {
            vec_remove_item(&mut *send_chunk_queue, &chunk_position);
        }
    }

    /// Send chunk which was just loaded
    pub fn send_loaded_chunk(&self, chunk_position: &ChunkPosition, message: ServerMessages) {
        if self.already_sended.read().contains(&chunk_position) {
            panic!("Tried to send already sended chunk! {}", chunk_position);
        }
        self.send_message(NetworkMessageType::WorldInfo, message);

        // Watch chunk
        self.already_sended.write().push(chunk_position.clone());
    }

    /// Send chunks to unload
    pub fn send_unload_chunks(&self, world_slug: &String, mut abandoned_chunks: Vec<ChunkPosition>) {
        if abandoned_chunks.len() == 0 {
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
        self.send_message(NetworkMessageType::ReliableOrdered, input);
    }

    pub fn send_message(&self, message_type: NetworkMessageType, message: ServerMessages) {
        let msg = SendClientMessageEvent::new(self.get_client_id().clone(), message_type, message);
        self.client_messages_output.0.send(msg).unwrap()
    }

    pub fn drain_client_messages(&self) -> Drain<SendClientMessageEvent> {
        self.client_messages_output.1.drain()
    }
}

impl Display for ClientNetwork {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let login = match self.client_info.as_ref() {
            Some(i) => i.get_login().clone(),
            None => "-".to_string(),
        };
        write!(f, "ip:{} login:{}", self.get_client_ip(), login)
    }
}

impl ConsoleSender for ClientNetwork {
    fn send_console_message(&self, message: String) {
        NetworkPlugin::send_console_output(&self, message);
    }
}
impl ConsoleSenderType for ClientNetwork {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
