use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    blocks::block_info::BlockInfo,
    chunks::{block_position::ChunkBlockPosition, chunk_position::ChunkPosition, utils::{PacketChunkSectionData, SectionsData}},
    VERTICAL_SECTIONS,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ClientMessages {
    ConnectionInfo { login: String },
    ConsoleInput { command: String },
    PlayerMove { position: [f32; 3], yaw: f32, pitch: f32 },
    ChunkRecieved { chunk_positions: Vec<ChunkPosition> },
}

pub type ChunkDataType = HashMap<ChunkBlockPosition, BlockInfo>;
pub type NetworkSectionsType = [PacketChunkSectionData; VERTICAL_SECTIONS];

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServerMessages {
    AllowConnection,
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
    ChunkSectionEncodedInfo {
        world_slug: String,
        chunk_position: ChunkPosition,
        sections: NetworkSectionsType,
    },
    ChunkSectionInfo {
        world_slug: String,
        chunk_position: ChunkPosition,
        sections: SectionsData,
    },
    UnloadChunks {
        world_slug: String,
        chunks: Vec<ChunkPosition>,
    },
}

pub enum NetworkMessageType {
    ReliableOrdered,
    ReliableUnordered,
    Unreliable,
}
