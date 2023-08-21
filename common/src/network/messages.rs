use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::chunks::chunk_position::ChunkPosition;

use super::NetworkSectionType;

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessages {
    ConsoleInput { command: String },
    PlayerMove {
        position: [f32; 3],
        yaw: f32,
        pitch: f32,
    },
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
        yaw: f32,
        pitch: f32,
    },
    ChunkSectionInfo {
        // x, z
        chunk_position: ChunkPosition,
        sections: NetworkSectionType,
    },
}
