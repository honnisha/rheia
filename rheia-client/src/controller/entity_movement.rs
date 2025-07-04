use common::chunks::rotation::Rotation;
use godot::{prelude::Vector3, register::GodotClass};
use network::messages::ClientMessages;
use std::fmt::{self, Display, Formatter};

use crate::utils::bridge::IntoNetworkVector;

/// Used to transmit motion data
#[derive(Clone, Copy, Debug, PartialEq, GodotClass)]
#[class(no_init)]
pub struct EntityMovement {
    position: Vector3,
    rotation: Rotation,
}

impl Display for EntityMovement {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "(pos:{} rotate:{})", self.position, self.rotation)
    }
}

impl EntityMovement {
    pub fn get_position(&self) -> &Vector3 {
        &self.position
    }

    pub fn create(position: Vector3, rotation: Rotation) -> Self {
        Self { position, rotation }
    }

    pub fn into_network(&self) -> ClientMessages {
        ClientMessages::PlayerMove {
            position: self.position.to_network(),
            rotation: self.rotation,
        }
    }
}
