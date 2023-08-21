use serde::{Deserialize, Serialize};

pub enum ClientReliableChannel {
    Messages,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessages {
    ConsoleInput { command: String },
}

impl From<ClientReliableChannel> for u8 {
    fn from(channel_id: ClientReliableChannel) -> Self {
        match channel_id {
            ClientReliableChannel::Messages => 0,
        }
    }
}
