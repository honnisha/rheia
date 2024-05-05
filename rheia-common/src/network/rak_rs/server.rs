use ahash::AHashMap;
use flume::{Drain, Receiver, Sender};
use parking_lot::RwLock;
use rak_rs::connection::Connection;
use rak_rs::Listener;
use std::sync::Arc;
use std::time::SystemTime;
use std::{thread, time};

use crate::network::messages::{ClientMessages, NetworkMessageType, ServerMessages};
use crate::network::server::{ConnectionMessages, ServerNetwork};

#[derive(Clone)]
pub struct RakNetClient {
    client_id: u64,
    channel_server_messages: (
        Sender<(ServerMessages, NetworkMessageType)>,
        Receiver<(ServerMessages, NetworkMessageType)>,
    ),
}

impl RakNetClient {
    fn new() -> Self {
        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let client_id = current_time.as_millis() as u64;
        Self {
            client_id,
            channel_server_messages: flume::unbounded(),
        }
    }

    fn send_message(&self, message: ServerMessages, message_type: NetworkMessageType) {
        self.channel_server_messages.0.send((message, message_type)).unwrap()
    }
}

#[derive(Clone)]
pub struct RakNetServerNetwork {
    clients: Arc<RwLock<AHashMap<u64, RakNetClient>>>,

    channel_client_messages: (Sender<(u64, ClientMessages)>, Receiver<(u64, ClientMessages)>),
    channel_connections: (Sender<ConnectionMessages>, Receiver<ConnectionMessages>),
    channel_errors: (Sender<String>, Receiver<String>),
}

impl RakNetServerNetwork {
    fn add_client(&self, client: RakNetClient) {
        self.clients.write().insert(client.client_id.clone(), client);
    }
}

impl ServerNetwork for RakNetServerNetwork {
    fn new(ip_port: String) -> Self {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        let network = RakNetServerNetwork {
            clients: Default::default(),
            channel_client_messages: flume::unbounded(),
            channel_connections: flume::unbounded(),
            channel_errors: flume::unbounded(),
        };

        let n = network.clone();
        thread::spawn(move || {
            runtime.block_on(async move {
                log::debug!(target: "raknet", "Network thread spawned successfully");

                let mut server = Listener::bind(ip_port).await.unwrap();
                server.start().await.unwrap();
                log::debug!(target: "raknet", "Server bined successfully");

                loop {
                    match server.accept().await {
                        Ok(c) => {
                            let client = RakNetClient::new();
                            n.add_client(client.clone());

                            let connection = ConnectionMessages::Connect {
                                client_id: client.client_id.clone(),
                                ip: c.address.ip().to_string(),
                            };
                            n.channel_connections.0.send(connection).unwrap();

                            tokio::task::spawn(handle(
                                c,
                                client,
                                n.channel_client_messages.0.clone(),
                                n.channel_connections.0.clone(),
                            ));
                        }
                        Err(e) => {
                            panic!("Connection error: {:?}", e);
                        }
                    }
                    thread::sleep(time::Duration::from_millis(50));
                }
            });
        });
        log::debug!(target: "raknet", "RakNetServerNetwork thread created");
        network
    }

    fn step(&self, _delta: std::time::Duration) -> bool {
        true
    }

    fn iter_client_messages(&self) -> Drain<(u64, ClientMessages)> {
        self.channel_client_messages.1.drain()
    }

    fn iter_connections(&self) -> Drain<ConnectionMessages> {
        self.channel_connections.1.drain()
    }

    fn iter_errors(&self) -> Drain<String> {
        self.channel_errors.1.drain()
    }

    fn is_connected(&self, client_id: u64) -> bool {
        let clients = self.clients.read();
        clients.contains_key(&client_id)
    }

    fn send_message(&self, client_id: u64, message: &ServerMessages, message_type: NetworkMessageType) {
        let clients = self.clients.read();
        match clients.get(&client_id) {
            Some(c) => {
                log::trace!(target: "raknet", "SEND client {} message: {:?}", client_id, message);
                c.send_message(message.clone(), message_type);
            }
            None => panic!("Sended server message to non existing client {}", client_id),
        }
    }
}

async fn handle(
    mut conn: Connection,
    client: RakNetClient,
    channel_client_messages: Sender<(u64, ClientMessages)>,
    channel_connections: Sender<ConnectionMessages>,
) {
    log::trace!(target: "raknet", "Handle started for client:{:?}", client.client_id);

    loop {
        if conn.is_closed().await {
            let message = ConnectionMessages::Disconnect {
                client_id: client.client_id.clone(),
                reason: "disconnet".to_string(),
            };
            channel_connections.send(message).unwrap();
            log::debug!(target: "raknet", "Disconnet client {}", client.client_id);
            break;
        }

        match conn.recv().await {
            Ok(client_message) => {
                let decoded: ClientMessages = match bincode::deserialize(&client_message) {
                    Ok(d) => d,
                    Err(e) => {
                        log::error!(target: "raknet", "Decode client message error: \"{}\" original: {:?}", e, client_message);
                        continue;
                    }
                };
                log::trace!(target: "raknet", "RECIEVED to client {} message: {:?}", client.client_id, decoded);
                channel_client_messages
                    .send((client.client_id.clone(), decoded))
                    .unwrap();
            },
            Err(e) => panic!("conn.recv() error: {:?}", e),
        }

        for (message, _message_type) in client.channel_server_messages.1.drain() {
            let encoded = bincode::serialize(&message).unwrap();
            conn.send(&encoded, false).await.unwrap();
        }
    }
}
