use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::{
    blocks::block_info::BlockInfo,
    chunks::{block_position::ChunkBlockPosition, chunk_position::ChunkPosition, utils::{PacketChunkSectionData, SectionsData}},
    VERTICAL_SECTIONS,
};


/// Network 3D vector
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Display for Vector3 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "x:{} y:{} z:{}", self.x, self.y, self.z)
    }
}
impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z }
    }

    pub fn zero() -> Self {
        Vector3 { x: 0.0, y: 0.0, z: 0.0 }
    }
}

pub trait IntoNetworkVector {
    fn to_network(&self) -> Vector3;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ClientMessages {
    ConnectionInfo { login: String },
    ConsoleInput { command: String },
    PlayerMove { position: Vector3, yaw: f32, pitch: f32 },
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
        location: Vector3,
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
