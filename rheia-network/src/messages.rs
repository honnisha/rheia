use std::collections::HashMap;

use common::blocks::block_type::BlockType;
use common::chunks::block_position::ChunkBlockPosition;
use common::chunks::chunk_position::ChunkPosition;
use common::chunks::position::Vector3;
use common::chunks::rotation::Rotation;
use common::VERTICAL_SECTIONS;
use common::{blocks::block_info::BlockInfo, chunks::block_position::BlockPosition};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Serialize, Deserialize, Clone, Display)]
pub enum ClientMessages {
    ConnectionInfo {
        login: String,
        version: String,
        architecture: String,
        rendering_device: String,
    },
    ConsoleInput {
        command: String,
    },
    PlayerMove {
        position: Vector3,
        rotation: Rotation,
    },
    ChunkRecieved {
        chunk_positions: Vec<ChunkPosition>,
    },
    EditBlockRequest {
        world_slug: String,
        position: BlockPosition,
        new_block_info: BlockInfo,
    },
    ResourcesLoaded {
        last_index: u32,
    },
    SettingsLoaded,
}

pub type ChunkDataType = HashMap<ChunkBlockPosition, BlockInfo>;
pub type SectionsData = [Box<ChunkDataType>; VERTICAL_SECTIONS];

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResurceScheme {
    pub slug: String,

    // Hash: name
    pub scripts: HashMap<String, String>,

    // Hash: name
    pub media: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display)]
pub enum ServerMessages {
    AllowConnection,
    ConsoleOutput {
        message: String,
    },

    // Server settings and resources
    ResourcesScheme {
        list: Vec<ResurceScheme>,
        archive_hash: u64,
    },
    ResourcesPart {
        index: u32,
        total: u32,
        data: Vec<u8>,
        last: bool,
    },
    Settings {
        block_types: HashMap<u32, BlockType>,
    },

    // Used to teleport the player's client controller.
    Teleport {
        world_slug: String,
        position: Vector3,
        rotation: Rotation,
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
    EditBlock {
        world_slug: String,
        position: BlockPosition,
        new_block_info: BlockInfo,
    },
}

pub enum NetworkMessageType {
    ReliableOrdered,
    ReliableUnordered,
    Unreliable,
    WorldInfo,
}
