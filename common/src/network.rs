use std::{collections::HashMap, time::Duration};

use bevy_renet::renet::{transport::NETCODE_KEY_BYTES, ChannelConfig, ConnectionConfig, SendType};
use renet::transport::NETCODE_USER_DATA_BYTES;
use serde::{Deserialize, Serialize};

use crate::{blocks::block_info::BlockInfo, VERTICAL_SECTIONS, chunks::{chunk_position::ChunkPosition, block_position::ChunkBlockPosition}};

pub const PRIVATE_KEY: &[u8; NETCODE_KEY_BYTES] = b"an example very very secret key."; // 32-bytes
pub const PROTOCOL_ID: u64 = 7;

pub enum ClientChannel {
    Messages,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessages {
    ConsoleInput { command: String },
}

impl From<ClientChannel> for u8 {
    fn from(channel_id: ClientChannel) -> Self {
        match channel_id {
            ClientChannel::Messages => 0,
        }
    }
}

impl ClientChannel {
    pub fn channels_config() -> Vec<ChannelConfig> {
        vec![ChannelConfig {
            channel_id: Self::Messages.into(),
            max_memory_usage_bytes: 5 * 1024 * 1024,
            send_type: SendType::ReliableOrdered {
                resend_time: Duration::ZERO,
            },
        }]
    }
}

pub enum ServerChannel {
    Messages,
}

pub type ChunkDataType = HashMap<ChunkBlockPosition, BlockInfo>;
pub type NetworkSectionType = [ChunkDataType; VERTICAL_SECTIONS];

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessages {
    ConsoleOutput {
        message: String,
    },
    Resource {
        slug: String,
        scripts: HashMap<String, String>,
    },
    Teleport {
        world_slug: String,
        location: [f32; 3],
    },
    ChunkSectionInfo {
        // x, z
        chunk_position: ChunkPosition,
        sections: NetworkSectionType,
    }
}

impl From<ServerChannel> for u8 {
    fn from(channel_id: ServerChannel) -> Self {
        match channel_id {
            ServerChannel::Messages => 0,
        }
    }
}

impl ServerChannel {
    pub fn channels_config() -> Vec<ChannelConfig> {
        vec![ChannelConfig {
            channel_id: Self::Messages.into(),
            max_memory_usage_bytes: 10 * 1024 * 1024,
            send_type: SendType::Unreliable,
        }]
    }
}

pub fn connection_config() -> ConnectionConfig {
    ConnectionConfig {
        available_bytes_per_tick: 1024 * 1024,
        client_channels_config: ClientChannel::channels_config(),
        server_channels_config: ServerChannel::channels_config(),
    }
}

pub struct Login(pub String);

impl Login {
    pub fn to_netcode_user_data(&self) -> [u8; NETCODE_USER_DATA_BYTES] {
        let mut user_data = [0u8; NETCODE_USER_DATA_BYTES];
        if self.0.len() > NETCODE_USER_DATA_BYTES - 8 {
            panic!("Login is too big");
        }
        user_data[0] = self.0.len() as u8;
        user_data[1..self.0.len() + 1].copy_from_slice(self.0.as_bytes());

        user_data
    }

    pub fn from_user_data(user_data: &[u8; NETCODE_USER_DATA_BYTES]) -> Self {
        let mut len = user_data[0] as usize;
        len = len.min(NETCODE_USER_DATA_BYTES - 1);
        let data = user_data[1..len + 1].to_vec();
        let login = String::from_utf8(data).unwrap_or("unknown".to_string());
        Self(login)
    }
}
