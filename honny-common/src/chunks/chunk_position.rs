use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, Hash)]
pub struct ChunkPosition {
    pub x: i64,
    pub z: i64,
}

impl ChunkPosition {
    pub const fn new(x: i64, z: i64) -> Self {
        Self { x, z }
    }
}

impl PartialEq for ChunkPosition {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.z == other.z
    }
}
impl Eq for ChunkPosition {}

impl Display for ChunkPosition {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "(x:{}, z:{})", self.x, self.z)
    }
}
