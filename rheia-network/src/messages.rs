use common::blocks::block_type::BlockType;
use common::chunks::block_position::BlockPosition;
use common::chunks::chunk_data::{BlockDataInfo, BlockIndexType, ChunkData};
use common::chunks::chunk_position::ChunkPosition;
use common::chunks::position::Vector3;
use common::chunks::rotation::Rotation;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use strum_macros::Display;

use crate::entities::entity_tag::EntityTag;
use crate::entities::{EntityNetworkComponent, EntitySkin};

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
        new_block_info: Option<BlockDataInfo>,
    },
    ResourcesHasCache {
        exists: bool,
    },
    ResourcesLoaded {
        last_index: u32,
    },
    SettingsLoaded,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResurceScheme {
    pub slug: String,

    // Hash: name
    pub scripts: HashMap<String, String>,

    // Hash: name
    pub media: HashMap<String, String>,
}

pub type NetworkEntityTag = EntityTag;
pub type NetworkEntitySkin = EntitySkin;

#[derive(Debug, Serialize, Deserialize, Clone, Display)]
pub enum ServerMessages {
    AllowConnection,
    ConsoleOutput {
        message: String,
    },
    Disconnect {
        message: Option<String>,
    },

    // Information about server resources (media, scripts)
    ResourcesScheme {
        list: Vec<ResurceScheme>,
        archive_hash: u64,
    },
    ResourcesPart {
        index: u32,
        total: u32,
        data: Vec<u8>,
    },
    Settings {
        block_types: Vec<BlockType>,
        block_id_map: BTreeMap<BlockIndexType, String>,
    },

    SpawnWorld {
        world_slug: String,
    },
    UpdatePlayerComponent {
        component: EntityNetworkComponent,
    },
    PlayerSpawn {
        world_slug: String,
        position: Vector3,
        rotation: Rotation,
        components: Vec<EntityNetworkComponent>,
    },
    ChunkSectionInfo {
        world_slug: String,
        chunk_position: ChunkPosition,
        sections: ChunkData,
    },
    ChunkSectionInfoEncoded {
        world_slug: String,
        chunk_position: ChunkPosition,
        encoded: Vec<u8>,
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
        components: Vec<EntityNetworkComponent>,
    },
    UpdateEntityComponent {
        world_slug: String,
        id: u32,
        component: EntityNetworkComponent,
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
        new_block_info: Option<BlockDataInfo>,
    },
}

pub enum NetworkMessageType {
    ReliableOrdered,
    ReliableUnordered,
    Unreliable,
    WorldInfo,
}
