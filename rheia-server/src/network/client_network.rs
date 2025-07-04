use bevy::prelude::{Component, Entity};
use common::{chunks::chunk_position::ChunkPosition, utils::vec_remove_item};
use core::fmt;
use network::{
    NetworkServerConnection,
    messages::{NetworkMessageType, ServerMessages},
    server::IServerConnection,
};
use parking_lot::{RwLock, RwLockReadGuard, lock_api::MappedRwLockReadGuard};
use std::{any::Any, fmt::Display, sync::Arc};

use crate::{
    SEND_CHUNK_QUEUE_LIMIT,
    console::console_sender::{ConsoleSender, ConsoleSenderType},
    entities::{
        EntityComponent,
        entity::{Position, Rotation},
    },
};

use super::{events::on_connection_info::PlayerConnectionInfoEvent, server::NetworkPlugin};

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
    _architecture: String,
    _rendering_device: String,
}

impl ClientInfo {
    pub fn new(event: &PlayerConnectionInfoEvent) -> Self {
        Self {
            login: event.login.clone(),
            version: event.version.clone(),
            _architecture: event.architecture.clone(),
            _rendering_device: event.rendering_device.clone(),
        }
    }

    pub fn get_login(&self) -> &String {
        &self.login
    }

    pub fn get_version(&self) -> &String {
        &self.version
    }

    pub fn _get_architecture(&self) -> &String {
        &self._architecture
    }
}

#[derive(Clone, Component)]
pub struct ClientNetwork {
    connection: NetworkServerConnection,

    client_info: Arc<RwLock<Option<ClientInfo>>>,

    // For fast finding player current world slug
    world_entity: Arc<RwLock<Option<WorldEntity>>>,

    // Current chunks that player can see
    // Tp prevent resend chunks
    already_sended: Arc<RwLock<Vec<ChunkPosition>>>,

    // Chunks was sended by network
    // but not yet recieved by the player
    send_chunk_queue: Arc<RwLock<Vec<ChunkPosition>>>,
}

impl ClientNetwork {
    pub fn new(connection: NetworkServerConnection) -> Self {
        ClientNetwork {
            connection,
            client_info: Default::default(),
            world_entity: Default::default(),
            already_sended: Default::default(),
            send_chunk_queue: Default::default(),
        }
    }

    pub(crate) fn get_connection(&self) -> &NetworkServerConnection {
        &self.connection
    }

    pub(crate) fn get_client_id(&self) -> u64 {
        self.connection.get_client_id()
    }

    pub fn send_allow_connection(&self) {
        self.send_message(NetworkMessageType::ReliableOrdered, &ServerMessages::AllowConnection {});
    }

    pub fn get_client_info(&self) -> Option<MappedRwLockReadGuard<'_, parking_lot::RawRwLock, ClientInfo>> {
        RwLockReadGuard::try_map(self.client_info.read(), |p| match p {
            Some(c) => Some(c),
            None => None,
        })
        .ok()
    }

    pub fn set_client_info(&self, info: ClientInfo) {
        let mut client_info = self.client_info.write();
        *client_info = Some(info);
    }

    pub fn get_client_ip(&self) -> &String {
        self.connection.get_ip()
    }

    /// Stores the player's information about which world he is in
    pub fn get_world_entity(&self) -> Option<WorldEntity> {
        let lock = self.world_entity.as_ref().read();
        lock.clone()
    }

    pub fn set_world_entity(&self, world_entity: Option<WorldEntity>) {
        *self.world_entity.write() = world_entity;
    }

    pub fn network_send_spawn(&self, position: &Position, rotation: &Rotation, components: &Vec<EntityComponent>) {
        let lock = self.get_world_entity();
        let world_entity = lock.as_ref().unwrap();
        let components = components.iter().map(|x| x.to_network()).collect::<Vec<_>>();
        let input = ServerMessages::PlayerSpawn {
            world_slug: world_entity.get_world_slug().clone(),
            position: position.to_network(),
            rotation: rotation.to_network(),
            components,
        };
        self.send_message(NetworkMessageType::ReliableOrdered, &input);
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
        self.send_message(NetworkMessageType::WorldInfo, &message);

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
        self.send_message(NetworkMessageType::ReliableOrdered, &input);
    }

    pub fn send_message(&self, message_type: NetworkMessageType, message: &ServerMessages) {
        self.connection.send_message(message_type, message);
    }

    pub fn send_disconnect(&mut self, message: Option<String>) {
        let msg = ServerMessages::Disconnect { message };
        self.connection
            .send_message(NetworkMessageType::ReliableUnordered, &msg);
        self.connection.disconnect();
    }
}

impl Display for ClientNetwork {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let login = match self.get_client_info() {
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
