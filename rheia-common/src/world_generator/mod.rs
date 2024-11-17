use std::collections::HashMap;

use crate::{blocks::block_info::BlockInfo, chunks::block_position::ChunkBlockPosition};

pub mod default;
pub mod sphere;

pub(crate) type ChunkDataType = HashMap<ChunkBlockPosition, BlockInfo>;
