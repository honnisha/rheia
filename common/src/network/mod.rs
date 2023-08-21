use std::collections::HashMap;

use renet::{ConnectionConfig, transport::NETCODE_KEY_BYTES};

use crate::{chunks::block_position::ChunkBlockPosition, blocks::block_info::BlockInfo, VERTICAL_SECTIONS};

use self::channels::{get_client_channels_config, get_server_channels_config};

pub mod login;
pub mod channels;
pub mod messages;

pub const PRIVATE_KEY: &[u8; NETCODE_KEY_BYTES] = b"an example very very secret key."; // 32-bytes
pub const PROTOCOL_ID: u64 = 7;

pub type ChunkDataType = HashMap<ChunkBlockPosition, BlockInfo>;
pub type NetworkSectionType = [Box<ChunkDataType>; VERTICAL_SECTIONS];

pub fn connection_config() -> ConnectionConfig {
    ConnectionConfig {
        available_bytes_per_tick: 1024 * 1024,
        client_channels_config: get_client_channels_config(),
        server_channels_config: get_server_channels_config(),
    }
}
