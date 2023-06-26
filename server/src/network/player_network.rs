use crate::console::console_sender::ConsoleSender;

pub struct PlayerNetwork {
    client_id: u64,
    login: String,
}

impl PlayerNetwork {
    pub fn new(client_id: u64, login: String) -> Self {
        PlayerNetwork {
            client_id,
            login,
        }
    }

    pub fn get_login(&self) -> &String {
        &self.login
    }
}

impl ConsoleSender for PlayerNetwork {
    fn send_console_message(&self, message: String) {

    }
}
