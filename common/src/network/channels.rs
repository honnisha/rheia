use std::time::Duration;

use renet::{ChannelConfig, SendType};

pub enum ClientChannel {
    Reliable,
    Unreliable,
}

impl From<ClientChannel> for u8 {
    fn from(channel_id: ClientChannel) -> Self {
        match channel_id {
            ClientChannel::Reliable => 0,
            ClientChannel::Unreliable => 1,
        }
    }
}

pub fn get_client_channels_config() -> Vec<ChannelConfig> {
    vec![
        ChannelConfig {
            channel_id: ClientChannel::Reliable.into(),
            max_memory_usage_bytes: 1 * 1024 * 1024,
            send_type: SendType::ReliableOrdered {
                resend_time: Duration::from_secs_f32(0.5_f32),
            },
        },
        ChannelConfig {
            channel_id: ClientChannel::Unreliable.into(),
            max_memory_usage_bytes: 1 * 1024 * 1024,
            send_type: SendType::Unreliable,
        },
    ]
}

pub enum ServerChannel {
    Reliable,
    Unreliable,
    Chunks,
}

impl From<ServerChannel> for u8 {
    fn from(channel_id: ServerChannel) -> Self {
        match channel_id {
            ServerChannel::Reliable => 0,
            ServerChannel::Unreliable => 1,
            ServerChannel::Chunks => 2,
        }
    }
}

pub fn get_server_channels_config() -> Vec<ChannelConfig> {
    vec![
        ChannelConfig {
            channel_id: ServerChannel::Reliable.into(),
            max_memory_usage_bytes: 1 * 1024 * 1024, // in MB
            send_type: SendType::ReliableOrdered {
                resend_time: Duration::from_secs_f32(0.5_f32),
            },
        },
        ChannelConfig {
            channel_id: ServerChannel::Unreliable.into(),
            max_memory_usage_bytes: 1 * 1024 * 1024,
            send_type: SendType::Unreliable,
        },
        ChannelConfig {
            channel_id: ServerChannel::Chunks.into(),
            max_memory_usage_bytes: 5 * 1024 * 1024,
            send_type: SendType::ReliableOrdered {
                resend_time: Duration::from_secs_f32(1.0_f32),
            },
        },
    ]
}
