use std::time::Duration;

use renet::{ChannelConfig, SendType};

pub enum ClientChannel {
    ReliableOrdered,
    ReliableUnordered,
    Unreliable,
}

impl From<ClientChannel> for u8 {
    fn from(channel_id: ClientChannel) -> Self {
        match channel_id {
            ClientChannel::ReliableOrdered => 0,
            ClientChannel::ReliableUnordered => 1,
            ClientChannel::Unreliable => 2,
        }
    }
}

pub fn get_client_channels_config() -> Vec<ChannelConfig> {
    vec![
        ChannelConfig {
            channel_id: ClientChannel::ReliableOrdered.into(),
            max_memory_usage_bytes: 1024 * 1024 * 5,
            send_type: SendType::ReliableOrdered {
                resend_time: Duration::from_secs_f32(0.5_f32),
            },
        },
        ChannelConfig {
            channel_id: ClientChannel::ReliableUnordered.into(),
            max_memory_usage_bytes: 1024 * 256,
            send_type: SendType::ReliableUnordered {
                resend_time: Duration::from_secs_f32(0.5_f32),
            },
        },
        ChannelConfig {
            channel_id: ClientChannel::Unreliable.into(),
            max_memory_usage_bytes: 1024 * 256,
            send_type: SendType::Unreliable,
        },
    ]
}

pub enum ServerChannel {
    ReliableOrdered,
    ReliableUnordered,
    Unreliable,
}

impl From<ServerChannel> for u8 {
    fn from(channel_id: ServerChannel) -> Self {
        match channel_id {
            ServerChannel::ReliableOrdered => 0,
            ServerChannel::ReliableUnordered => 1,
            ServerChannel::Unreliable => 2,
        }
    }
}

pub fn get_server_channels_config() -> Vec<ChannelConfig> {
    vec![
        ChannelConfig {
            channel_id: ServerChannel::ReliableOrdered.into(),
            max_memory_usage_bytes: 1024 * 1024 * 5,
            send_type: SendType::ReliableOrdered {
                resend_time: Duration::from_secs_f32(0.5_f32),
            },
        },
        ChannelConfig {
            channel_id: ServerChannel::ReliableUnordered.into(),
            max_memory_usage_bytes: 1024 * 256,
            send_type: SendType::ReliableUnordered {
                resend_time: Duration::from_secs_f32(0.5_f32),
            },
        },
        ChannelConfig {
            channel_id: ServerChannel::Unreliable.into(),
            max_memory_usage_bytes: 1024 * 256,
            send_type: SendType::Unreliable,
        },
    ]
}
