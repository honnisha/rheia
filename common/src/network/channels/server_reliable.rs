use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{chunks::chunk_position::ChunkPosition, network::NetworkSectionType};

pub enum ServerReliableChannel {
    Messages,
}

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
    },
}

impl From<ServerReliableChannel> for u8 {
    fn from(channel_id: ServerReliableChannel) -> Self {
        match channel_id {
            ServerReliableChannel::Messages => 0,
        }
    }
}
