use std::time::Duration;

use renet::{ChannelConfig, SendType};
use strum_macros::{Display, EnumIter};

#[derive(Display, EnumIter, Clone, Copy)]
pub enum ClientChannel {
    ReliableOrdered,
    ReliableUnordered,
    Unreliable,
    World,
}

impl From<ClientChannel> for u8 {
    fn from(channel_id: ClientChannel) -> Self {
        match channel_id {
            ClientChannel::ReliableOrdered => 0,
            ClientChannel::ReliableUnordered => 1,
            ClientChannel::Unreliable => 2,
            ClientChannel::World => 3,
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
            max_memory_usage_bytes: 1024 * 1024 * 5,
            send_type: SendType::ReliableUnordered {
                resend_time: Duration::from_secs_f32(0.5_f32),
            },
        },
        ChannelConfig {
            channel_id: ClientChannel::Unreliable.into(),
            max_memory_usage_bytes: 1024 * 256,
            send_type: SendType::Unreliable,
        },
        ChannelConfig {
            channel_id: ClientChannel::World.into(),
            max_memory_usage_bytes: 1024 * 1024 * 5,
            send_type: SendType::ReliableOrdered {
                resend_time: Duration::from_secs_f32(0.5_f32),
            },
        },
    ]
}

#[derive(Display, EnumIter, Clone, Copy)]
pub enum ServerChannel {
    ReliableOrdered,
    ReliableUnordered,
    Unreliable,
    World,
}

impl From<ServerChannel> for u8 {
    fn from(channel_id: ServerChannel) -> Self {
        match channel_id {
            ServerChannel::ReliableOrdered => 0,
            ServerChannel::ReliableUnordered => 1,
            ServerChannel::Unreliable => 2,
            ServerChannel::World => 3,
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
            max_memory_usage_bytes: 1024 * 1024 * 5,
            send_type: SendType::ReliableUnordered {
                resend_time: Duration::from_secs_f32(0.5_f32),
            },
        },
        ChannelConfig {
            channel_id: ServerChannel::Unreliable.into(),
            max_memory_usage_bytes: 1024 * 256,
            send_type: SendType::Unreliable,
        },
        ChannelConfig {
            channel_id: ServerChannel::World.into(),
            max_memory_usage_bytes: 1024 * 1024 * 5,
            send_type: SendType::ReliableOrdered {
                resend_time: Duration::from_secs_f32(0.5_f32),
            },
        },
    ]
}
