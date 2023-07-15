use std::fmt::{Display, Formatter, self};

use ahash::AHashMap;
use common::VERTICAL_SECTIONS;
use serde::{Serialize, Deserialize};

use super::chunk_section::ChunkSection;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ChunkPosition {
    pub x: i32,
    pub z: i32,
}

impl ChunkPosition {
    pub const fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }
}

impl Display for ChunkPosition {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "({}, {})", self.x, self.z)
    }
}

/// Container of 2d ChunkColumn's
#[derive(Default)]
pub struct ChunkMap {
    chunks: AHashMap<ChunkPosition, ChunkColumn>,
}

impl ChunkMap {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct ChunkColumn {
    sections: [Option<ChunkSection>; VERTICAL_SECTIONS + 2],
}
