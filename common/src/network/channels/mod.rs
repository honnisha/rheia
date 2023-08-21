use std::time::Duration;

use renet::{ChannelConfig, SendType};

use crate::network::channels::{clinet_reliable::ClientReliableChannel, server_reliable::ServerReliableChannel};

pub mod clinet_reliable;
pub mod server_reliable;

pub fn get_client_channels_config() -> Vec<ChannelConfig> {
    vec![ChannelConfig {
        channel_id: ClientReliableChannel::Messages.into(),
        max_memory_usage_bytes: 10 * 1024 * 1024,
        send_type: SendType::ReliableOrdered {
            resend_time: Duration::from_secs_f32(0.5_f32),
        },
    }]
}

pub fn get_server_channels_config() -> Vec<ChannelConfig> {
    vec![ChannelConfig {
        channel_id: ServerReliableChannel::Messages.into(),
        max_memory_usage_bytes: 10 * 1024 * 1024,
        send_type: SendType::ReliableOrdered {
            resend_time: Duration::from_secs_f32(0.5_f32),
        },
    }]
}
