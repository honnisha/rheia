use bevy_ecs::prelude::Bundle;
use bincode::Options;
use common::network_messages::ServerMessages;

use crate::{console::console_sender::ConsoleSender, console_send};

use super::server::NetworkServer;

#[derive(Bundle)]
pub struct PlayerNetwork {
    login: String,
    client_id: u64,
}

impl PlayerNetwork {
    pub fn init(login: String, client_id: u64) -> Self {
        PlayerNetwork {
            login: login,
            client_id: client_id,
        }
    }

    pub fn get_login(&self) -> &String {
        &self.login
    }
}

impl ConsoleSender for PlayerNetwork {
    fn get_name(&self) -> &String {
        &self.login
    }

    fn send_console_message(&self, message: String) {
        match bincode::options().serialize(&ServerMessages::ConsoleOutput { text: message }) {
            Ok(message) => NetworkServer::send_console_message(self.client_id, message),
            Err(e) => {
                console_send(format!(
                    "Error console message for login:{} error: {:?}",
                    self.get_login(),
                    e
                ));
            }
        }
    }
}
