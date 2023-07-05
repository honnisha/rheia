use core::fmt;
use std::fmt::Display;

use crate::console::console_sender::{ConsoleSender, ConsoleSenderType};

use super::server::NetworkPlugin;

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
