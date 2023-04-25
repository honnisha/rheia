use bincode::Options;
use common::network_messages::ServerMessages;

use crate::console::console_handler::{Console, ConsoleSender};

use super::server::NetworkServer;

pub struct Player {
    login: String,
    client_id: u64,
}

impl Player {
    pub fn init(login: String, client_id: u64) -> Self {
        Player {
            login: login,
            client_id: client_id,
        }
    }

    pub fn get_login(&self) -> &String {
        &self.login
    }
}

impl ConsoleSender for Player {
    fn get_name(&self) -> &String {
        &self.login
    }

    fn send_console_message(&self, message: String) {
        match bincode::options().serialize(&ServerMessages::ConsoleOutput { text: message }) {
            Ok(message) => NetworkServer::send_console_message(self.client_id, message),
            Err(e) => {
                Console::send_message(format!(
                    "Error console message for login:{} error: {:?}",
                    self.get_login(),
                    e
                ));
            }
        }
    }
}
