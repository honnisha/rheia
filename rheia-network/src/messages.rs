use std::collections::HashMap;

use common::chunks::block_position::ChunkBlockPosition;
use common::chunks::chunk_position::ChunkPosition;
use common::chunks::rotation::Rotation;
use common::VERTICAL_SECTIONS;
use common::{blocks::block_info::BlockInfo, chunks::block_position::BlockPosition};
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use common::chunks::position::Vector3;

use crate::utils::{PacketChunkSectionData, SectionsData};

#[derive(Debug, Serialize, Deserialize, Clone, Display)]
pub enum ClientMessages {
    ConnectionInfo {
        login: String,
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
    ResourceMedia {
        resurce_slug: String,
        name: String,
        data: Vec<u8>,
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
