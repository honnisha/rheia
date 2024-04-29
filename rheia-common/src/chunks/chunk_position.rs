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

    pub fn get_distance(&self, target: &ChunkPosition) -> f32 {
        ((target.x as f32 - self.x as f32).powf(2.0) + (target.z as f32 - self.z as f32).powf(2.0)).sqrt()
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

#[cfg(test)]
mod tests {
    use super::ChunkPosition;

    #[test]
    fn test_chunks_distance() {
        let distance = ChunkPosition::new(1, 2).get_distance(&ChunkPosition::new(20, 10));
        assert_eq!(distance, 20.615528);
    }
}
