use common::network::messages::ClientMessages;
use godot::{prelude::Vector3, register::GodotClass};
use std::fmt::{self, Display, Formatter};

use crate::{main_scene::FloatType, utils::bridge::IntoNetworkVector};

/// Used to transmit motion data
#[derive(Clone, Copy, Debug, PartialEq, GodotClass)]
#[class(init)]
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
            position: self.position.to_network(),
            yaw: self.yaw,
            pitch: self.pitch,
        }
    }
}
