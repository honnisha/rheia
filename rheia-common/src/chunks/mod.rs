use std::collections::HashMap;

use block_position::ChunkBlockPosition;

use crate::{blocks::block_info::BlockInfo, VERTICAL_SECTIONS};

pub mod block_position;
pub mod chunk_position;
pub mod position;
pub mod rotation;

pub type ChunkDataType = HashMap<ChunkBlockPosition, BlockInfo>;
pub type SectionsData = [Box<ChunkDataType>; VERTICAL_SECTIONS];
