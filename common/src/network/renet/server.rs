use flume::{Receiver, Sender, Drain};
use std::{
    net::UdpSocket,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
    time::{Duration, SystemTime},
};

use renet::{
    transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
    RenetServer, ServerEvent,
};

use super::{connection_config, PROTOCOL_ID, channels::{ClientChannel, ServerChannel}, login::Login};
use crate::network::{messages::{ClientMessages, ServerMessages, NetworkMessageType}, server::{ServerNetwork, ConnectionMessages}};
use log::error;

pub type ServerLock = Arc<RwLock<RenetServer>>;
pub type TransferLock = Arc<RwLock<NetcodeServerTransport>>;

pub struct RenetServerNetwork {
    server: ServerLock,
    transport: TransferLock,
    channel_client_messages: (Sender<(u64, ClientMessages)>, Receiver<(u64, ClientMessages)>),
    channel_connections: (Sender<ConnectionMessages>, Receiver<ConnectionMessages>),
}

impl RenetServerNetwork {
    pub fn get_server(&self) -> RwLockReadGuard<RenetServer> {
        self.server.as_ref().read().expect("poisoned")
    }

    fn get_server_mut(&self) -> RwLockWriteGuard<RenetServer> {
        self.server.as_ref().write().expect("poisoned")
    }

    pub fn get_transport(&self) -> RwLockReadGuard<NetcodeServerTransport> {
        self.transport.as_ref().read().expect("poisoned")
    }

    fn get_transport_mut(&self) -> RwLockWriteGuard<NetcodeServerTransport> {
        self.transport.as_ref().write().expect("poisoned")
    }

    fn map_type_channel(message_type: NetworkMessageType) -> ServerChannel {
        match message_type {
            NetworkMessageType::Chunks => ServerChannel::Chunks,
            NetworkMessageType::Movement => ServerChannel::Unreliable,
            NetworkMessageType::Message => ServerChannel::Reliable,
        }
    }
}

impl ServerNetwork for RenetServerNetwork {
    fn new(ip_port: String) -> Self {
        let server = RenetServer::new(connection_config());

        let public_addr = ip_port.parse().unwrap();
        let socket = UdpSocket::bind(public_addr).unwrap();
        let server_config = ServerConfig {
            max_clients: 64,
            protocol_id: PROTOCOL_ID,
            public_addr,
            authentication: ServerAuthentication::Unsecure,
        };
        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

        let transport = NetcodeServerTransport::new(current_time, server_config, socket).unwrap();
        let network = RenetServerNetwork {
            server: Arc::new(RwLock::new(server)),
            transport: Arc::new(RwLock::new(transport)),
            channel_client_messages: flume::unbounded(),
            channel_connections: flume::unbounded(),
        };
        network
    }

    fn step(&self, delta: Duration) {
        let mut server = self.get_server_mut();
        let mut transport = self.get_transport_mut();
        server.update(delta);

        if let Err(e) = transport.update(delta, &mut server) {
            error!("Transport error: {}", e.to_string());
        }

        for client_id in server.clients_id().into_iter() {
            while let Some(client_message) = server.receive_message(client_id, ClientChannel::Reliable) {
                let decoded: ClientMessages = match bincode::deserialize(&client_message) {
                    Ok(d) => d,
                    Err(e) => {
                        error!("Decode client reliable message error: {}", e);
                        continue;
                    }
                };
                self.channel_client_messages.0.send((client_id.clone(), decoded)).unwrap();
            }
            while let Some(client_message) = server.receive_message(client_id, ClientChannel::Unreliable) {
                let decoded: ClientMessages = match bincode::deserialize(&client_message) {
                    Ok(d) => d,
                    Err(e) => {
                        error!("Decode client unreliable message error: {}", e);
                        continue;
                    }
                };
                self.channel_client_messages.0.send((client_id.clone(), decoded)).unwrap();
            }
        }

        while let Some(event) = server.get_event() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    let user_data = transport.user_data(client_id.clone()).unwrap();
                    let login = Login::from_user_data(&user_data).0;
                    let connect = ConnectionMessages::Connect {
                        client_id: client_id.clone(),
                        login: login,
                    };
                    self.channel_connections.0.send(connect).unwrap();
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    let connect = ConnectionMessages::Disconnect {
                        client_id: client_id.clone(),
                        reason: reason.to_string(),
                    };
                    self.channel_connections.0.send(connect).unwrap();
                }
            }
            //server_events.send(event);
        }

        transport.send_packets(&mut server);
    }

    fn iter_client_messages(&self) -> Drain<(u64, ClientMessages)> {
        self.channel_client_messages.1.drain()
    }

    fn iter_connections(&self) -> Drain<ConnectionMessages> {
        self.channel_connections.1.drain()
    }

    fn is_connected(&self, client_id: u64) -> bool {
        self.get_server().is_connected(client_id)
    }

    fn send_message(&self, client_id: u64, message: &ServerMessages, message_type: NetworkMessageType) {
        let encoded = bincode::serialize(message).unwrap();
        self.get_server_mut().send_message(client_id, RenetServerNetwork::map_type_channel(message_type), encoded);
    }
}
