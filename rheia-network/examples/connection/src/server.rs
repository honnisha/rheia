use std::{collections::HashMap, time::Duration};

use network::{
    messages::{ClientMessages, NetworkMessageType, ServerMessages},
    server::{ConnectionMessages, IServerConnection, IServerNetwork},
    NetworkServer, NetworkServerConnection,
};

use crate::console::Console;

pub struct ClientNetwork {
    connection: NetworkServerConnection,
    login: Option<String>,
}

impl ClientNetwork {
    pub fn new(connection: NetworkServerConnection) -> Self {
        Self {
            connection,
            login: Default::default(),
        }
    }

    pub fn send_message(&self, message_type: NetworkMessageType, message: &ServerMessages) {
        self.connection.send_message(message_type, message);
    }
}

pub struct Server {
    server: NetworkServer,
    connections: HashMap<u64, ClientNetwork>,
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
            let delta = Duration::from_millis(10);
            self.server.step(delta.clone()).await;

            for message in self.server.drain_errors() {
                panic!("Network error: {}", message);
            }

            for (client_id, decoded) in self.server.drain_client_messages() {
                match decoded {
                    ClientMessages::ConsoleInput { command } => {
                        let Some(connection) = self.connections.get(&client_id) else {
                            continue;
                        };
                        let Some(login) = connection.login.as_ref() else {
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
                        let Some(connection) = self.connections.get_mut(&client_id) else {
                            continue;
                        };
                        connection.login = Some(login.clone());
                        log::info!("Connected login:{} ip:{}", client_id, login);
                    }
                    _ => unimplemented!(),
                }
            }
            for message in self.server.drain_connections() {
                match message {
                    ConnectionMessages::Connect { connection } => {
                        connection.send_message(NetworkMessageType::ReliableOrdered, &ServerMessages::AllowConnection);
                        let client_network = ClientNetwork::new(connection.clone());
                        self.connections.insert(connection.get_client_id(), client_network);
                    }
                    ConnectionMessages::Disconnect { client_id, reason } => {
                        self.connections.remove(&client_id);
                        log::info!("- Disconnected client_id:{} reason:{}", client_id, reason);
                    }
                }
            }

            for input in Console::get_input() {
                log::info!("- Console: {}", input);
                self.send_for_all(format!("Console: {}", input));
            }

            tokio::time::sleep(delta).await;
        }
    }

    pub fn send_for_all(&self, input: String) {
        let msg = ServerMessages::ConsoleOutput { message: input };
        for cleint_network in self.connections.values() {
            cleint_network.send_message(NetworkMessageType::ReliableOrdered, &msg);
        }
    }
}
