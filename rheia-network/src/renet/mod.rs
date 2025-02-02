use renet::ConnectionConfig;

use self::channels::{get_client_channels_config, get_server_channels_config};

pub mod client;
pub mod server;
pub mod channels;

pub const PROTOCOL_ID: u64 = 7;

pub fn connection_config() -> ConnectionConfig {
    ConnectionConfig {
        available_bytes_per_tick: 1024 * 1024,
        client_channels_config: get_client_channels_config(),
        server_channels_config: get_server_channels_config(),
    }
}
