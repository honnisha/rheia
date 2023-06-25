use crate::console::console_sender::ConsoleSender;

pub struct PlayerNetwork {
    client_id: u64,
}

impl PlayerNetwork {
    pub fn new(client_id: u64) -> Self {
        PlayerNetwork {
            client_id,
        }
    }
}

impl ConsoleSender for PlayerNetwork {
    fn send_console_message(&self, message: String) {

    }
}
