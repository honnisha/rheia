use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::{
    blocks::block_info::BlockInfo,
    chunks::{
        block_position::ChunkBlockPosition,
        chunk_position::ChunkPosition,
        utils::{PacketChunkSectionData, SectionsData},
    },
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
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
}

pub(crate) trait IntoNetworkVector {
    fn to_network(&self) -> Vector3;
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub struct Rotation {
    // vertical angle
    pub yaw: f32,

    // horizontal angle
    pub pitch: f32,
}
impl Display for Rotation {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "yaw:{} pitch:{}", self.yaw, self.pitch)
    }
}
impl PartialEq for Rotation {
    fn eq(&self, other: &Rotation) -> bool {
        self.yaw == other.yaw && self.pitch == other.pitch
    }
}

impl Rotation {
    pub fn new(yaw: f32, pitch: f32) -> Self {
        Self { yaw, pitch }
    }

    pub fn zero() -> Self {
        Self { yaw: 0.0, pitch: 0.0 }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Display)]
pub enum ClientMessages {
    ConnectionInfo { login: String },
    ConsoleInput { command: String },
    PlayerMove { position: Vector3, rotation: Rotation },
    ChunkRecieved { chunk_positions: Vec<ChunkPosition> },
}

pub type ChunkDataType = HashMap<ChunkBlockPosition, BlockInfo>;
pub type NetworkSectionsType = [PacketChunkSectionData; VERTICAL_SECTIONS];

#[derive(Debug, Serialize, Deserialize, Clone, Display)]
pub enum ServerMessages {
    AllowConnection,
    ConsoleOutput {
        message: String,
    },
    Resource {
        slug: String,
        scripts: HashMap<String, String>,
    },
    // Used to teleport the player's client controller.
    Teleport {
        world_slug: String,
        position: Vector3,
        rotation: Rotation,
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
    // In case the entity gets in the player's line of sight
    StartStreamingEntity {
        world_slug: String,
        id: u32,
        position: Vector3,
        rotation: Rotation,
    },
    // In case the entity escapes from the visible chunk or is deleted
    StopStreamingEntities {
        world_slug: String,
        ids: Vec<u32>,
    },
    EntityMove {
        world_slug: String,
        id: u32,
        position: Vector3,
        rotation: Rotation,
    },
}

pub enum NetworkMessageType {
    ReliableOrdered,
    ReliableUnordered,
    Unreliable,
}
