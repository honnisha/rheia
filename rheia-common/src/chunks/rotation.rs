use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub struct Rotation {
    /// Horisontal degrees (left-right) Y
    pub yaw: f32,

    // Vertical degrees (up-down) X
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
