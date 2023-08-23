use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    blocks::block_info::BlockInfo,
    chunks::{block_position::ChunkBlockPosition, chunk_position::ChunkPosition, utils::PacketChunkSectionData},
    VERTICAL_SECTIONS,
};

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessages {
    ConsoleInput { command: String },
    PlayerMove { position: [f32; 3], yaw: f32, pitch: f32 },
}

pub type ChunkDataType = HashMap<ChunkBlockPosition, BlockInfo>;
pub type NetworkSectionsType = [PacketChunkSectionData; VERTICAL_SECTIONS];

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
        chunk_position: ChunkPosition,
        sections: NetworkSectionsType,
    },
    UnloadChunks {
        chunks: Vec<ChunkPosition>,
    },
}
