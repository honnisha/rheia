use bevy_ecs::prelude::Component;
use common::chunks::position::Vector3 as NetworkVector3;
use common::chunks::rotation::Rotation as NetworkRotation;
use common::{
    chunks::{block_position::BlockPositionTrait, chunk_position::ChunkPosition},
    utils::fix_chunk_loc_pos,
};

pub type PositionFloatType = f32;

#[derive(Component, Clone, Copy, Default)]
pub struct Position {
    x: PositionFloatType,
    y: PositionFloatType,
    z: PositionFloatType,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Position {
    pub fn new(x: PositionFloatType, y: PositionFloatType, z: PositionFloatType) -> Self {
        Self { x, y, z }
    }

    pub fn to_network(&self) -> NetworkVector3 {
        NetworkVector3::new(self.x, self.y, self.z)
    }
}

impl BlockPositionTrait for Position {
    fn get_chunk_position(&self) -> ChunkPosition {
        ChunkPosition::new(fix_chunk_loc_pos(self.x as i64), fix_chunk_loc_pos(self.z as i64))
    }
}

pub trait IntoServerPosition {
    fn to_server(&self) -> Position;
}

impl IntoServerPosition for NetworkVector3 {
    fn to_server(&self) -> Position {
        Position::new(self.x, self.y, self.z)
    }
}

#[derive(Component, Clone, Copy, Default)]
pub struct Rotation {
    pitch: PositionFloatType,
    yaw: PositionFloatType,
}

impl Rotation {
    pub fn new(pitch: PositionFloatType, yaw: PositionFloatType) -> Self {
        Self { pitch, yaw }
    }

    pub fn _get_yaw(&self) -> &PositionFloatType {
        &self.yaw
    }

    pub fn _get_pitch(&self) -> &PositionFloatType {
        &self.pitch
    }

    pub fn to_network(&self) -> NetworkRotation {
        NetworkRotation::new(self.yaw, self.pitch)
    }
}

pub trait IntoServerRotation {
    fn to_server(&self) -> Rotation;
}

impl IntoServerRotation for NetworkRotation {
    fn to_server(&self) -> Rotation {
        Rotation::new(self.yaw, self.pitch)
    }
}
