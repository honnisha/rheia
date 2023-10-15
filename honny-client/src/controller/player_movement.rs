use common::{
    chunks::{
        block_position::{BlockPosition, BlockPositionTrait},
        chunk_position::ChunkPosition,
    },
    network::messages::ClientMessages,
};
use godot::prelude::{FromVariant, ToVariant, Vector3};
use std::fmt::{self, Display, Formatter};

use crate::{entities::position::GodotPositionConverter, main_scene::FloatType};

#[derive(Clone, Copy, Debug, PartialEq, ToVariant, FromVariant)]
pub struct PlayerMovement {
    // Player object position
    position: Vector3,

    // vertical angle
    yaw: FloatType,

    // horizontal angle
    pitch: FloatType,
}

impl Display for PlayerMovement {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "(pos:{} yaw:{} pitch:{})", self.position, self.yaw, self.pitch)
    }
}

impl PlayerMovement {
    pub fn create(position: Vector3, yaw: FloatType, pitch: FloatType) -> Self {
        Self { position, yaw, pitch }
    }

    pub fn into_network(&self) -> ClientMessages {
        ClientMessages::PlayerMove {
            position: GodotPositionConverter::vec3_to_array(&self.position),
            yaw: self.yaw,
            pitch: self.pitch,
        }
    }

    pub fn get_chunk_position(&self) -> ChunkPosition {
        BlockPosition::new(self.position.x as i64, self.position.y as i64, self.position.z as i64).get_chunk_position()
    }
}
