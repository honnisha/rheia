use std::time::Duration;

use network::{
    client::IClientNetwork,
    messages::{ClientMessages, NetworkMessageType, ServerMessages},
    NetworkClient,
};

use crate::console::Console;

pub struct Client {
    client: NetworkClient,
    login: String,
}

impl Client {
    pub async fn create(ip_port: String, login: String) -> Self {
        log::info!("Connecting to {}...", ip_port);
        let client = NetworkClient::new(ip_port.clone()).await.unwrap();
        log::info!("Client started; Listening on: {}", ip_port);
        Self { client, login }
    }

    pub async fn run(&mut self) {
        loop {
            let delta = Duration::from_millis(10);
            self.client.step(delta.clone()).await;
            // Recieve errors from network thread
            for error in self.client.iter_errors() {
                log::info!("Network error: {}", error);
                return;
            }

            for message in self.client.iter_server_messages() {
                match message {
                    ServerMessages::AllowConnection => {
                        let msg = ClientMessages::ConnectionInfo {
                            login: self.login.clone(),
                            version: String::from("-"),
                            architecture: String::from("-"),
                            rendering_device: String::from("-"),
                        };
                        self.client.send_message(NetworkMessageType::ReliableOrdered, &msg);
                    }
                    ServerMessages::ConsoleOutput { message } => {
                        log::info!("- {}", message);
                    }
                    _ => {
                        log::info!("unimplemented message: {:?}", message);
                        return;
                    }
                }
            }

            for input in Console::get_input() {
                let msg = ClientMessages::ConsoleInput { command: input };
                self.client.send_message(NetworkMessageType::ReliableOrdered, &msg);
            }

            tokio::time::sleep(delta).await;
        }
    }
}
