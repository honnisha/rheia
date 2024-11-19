use std::{collections::HashMap, time::Duration};

use network::{
    messages::{ClientMessages, NetworkMessageType, ServerMessages},
    server::{ConnectionMessages, IServerNetwork},
    NetworkServer,
};

use crate::console::Console;

pub struct Server {
    server: NetworkServer,
    connections: HashMap<u64, String>,
}

impl Server {
    pub async fn create(ip_port: String) -> Self {
        log::info!("Server started; Listening on: {}", ip_port);
        let server = NetworkServer::new(ip_port).await;
        Self {
            server,
            connections: Default::default(),
        }
    }

    pub async fn run(&mut self) {
        loop {
            let delta = Duration::from_secs_f32(0.1);
            self.server.step(delta.clone()).await;

            for message in self.server.drain_errors() {
                panic!("Network error: {}", message);
            }

            for (client_id, decoded) in self.server.drain_client_messages() {
                match decoded {
                    ClientMessages::ConsoleInput { command } => {
                        let Some(login) = self.connections.get(&client_id) else {
                            continue;
                        };
                        log::info!("- {}: {}", login, command);
                        self.send_for_all(format!("{}: {}", login, command));
                    }
                    ClientMessages::ConnectionInfo {
                        login,
                        version: _,
                        architecture: _,
                        rendering_device: _,
                    } => {
                        self.connections.insert(client_id.clone(), login.clone());
                        log::info!("Connected login:{} ip:{}", client_id, login);
                    }
                    _ => unimplemented!(),
                }
            }
            for message in self.server.drain_connections() {
                match message {
                    ConnectionMessages::Connect { client_id, ip: _ } => {
                        self.server.send_message(
                            client_id,
                            NetworkMessageType::ReliableOrdered,
                            &ServerMessages::AllowConnection,
                        );
                    }
                    ConnectionMessages::Disconnect { client_id, reason } => {
                        self.connections.remove(&client_id);
                        log::info!("- Disconnected client_id:{} reason:{}", client_id, reason);
                    }
                }
            }

            for input in Console::get_input() {
                self.send_for_all(format!("Console: {}", input));
            }

            tokio::time::sleep(delta).await;
        }
    }

    pub fn send_for_all(&self, input: String) {
        let msg = ServerMessages::ConsoleOutput { message: input };
        for client_id in self.connections.keys() {
            self.server
                .send_message(*client_id, NetworkMessageType::ReliableOrdered, &msg);
        }
    }
}
