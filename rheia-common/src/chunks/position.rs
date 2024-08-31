use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    ops::Add,
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

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

pub trait IntoNetworkVector {
    fn to_network(&self) -> Vector3;
}
