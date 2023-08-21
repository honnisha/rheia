use core::fmt;
use std::fmt::Display;

use common::network::{messages::ServerMessages, channels::ServerChannel};

use crate::{console::console_sender::{ConsoleSender, ConsoleSenderType}, entities::entity::Position};

use super::server::{NetworkPlugin, NetworkContainer};

#[derive(Clone)]
pub struct PlayerNetwork {
    client_id: u64,
    login: String,

    // For fast finding player current world slug
    pub current_world: Option<String>,
}

impl PlayerNetwork {
    pub fn new(client_id: u64, login: String) -> Self {
        PlayerNetwork {
            client_id,
            login,
            current_world: None,
        }
    }

    pub fn get_login(&self) -> &String {
        &self.login
    }

    pub fn get_client_id(&self) -> &u64 {
        &self.client_id
    }

    pub fn send_teleport(
        &mut self,
        network_container: &NetworkContainer,
        world_slug: &String,
        position: &Position,
        yaw: f32,
        pitch: f32,
    ) {
        let mut server = network_container.server.write().expect("poisoned");
        let input = ServerMessages::Teleport {
            world_slug: world_slug.clone(),
            location: position.to_array(),
            yaw,
            pitch,
        };
        let encoded = bincode::serialize(&input).unwrap();
        server.send_message(self.client_id.clone(), ServerChannel::Reliable, encoded)
    }
}

impl Display for PlayerNetwork {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.login)
    }
}

impl ConsoleSender for PlayerNetwork {
    fn send_console_message(&self, message: String) {
        NetworkPlugin::send_console_output(self.client_id.clone(), message);
    }
}
impl ConsoleSenderType for PlayerNetwork {}
