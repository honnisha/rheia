use flume::{Receiver, Sender};
use std::{
    net::UdpSocket,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
    time::{Duration, SystemTime},
};
use strum::IntoEnumIterator;

use renet::{
    transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
    ClientId, RenetServer, ServerEvent,
};

use crate::{
    messages::{ClientMessages, NetworkMessageType, ServerMessages},
    server::{ConnectionMessages, IServerConnection, IServerNetwork},
};

use super::{
    channels::{ClientChannel, ServerChannel},
    connection_config, PROTOCOL_ID,
};

type ServerLock = Arc<RwLock<RenetServer>>;
type TransferLock = Arc<RwLock<NetcodeServerTransport>>;

pub struct RenetServerNetwork {
    server: ServerLock,
    transport: TransferLock,
    channel_client_messages: (Sender<(u64, ClientMessages)>, Receiver<(u64, ClientMessages)>),
    channel_connections: (
        Sender<ConnectionMessages<RenetServerConnection>>,
        Receiver<ConnectionMessages<RenetServerConnection>>,
    ),
    channel_errors: (Sender<String>, Receiver<String>),
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
            NetworkMessageType::ReliableOrdered => ServerChannel::ReliableOrdered,
            NetworkMessageType::Unreliable => ServerChannel::Unreliable,
            NetworkMessageType::ReliableUnordered => ServerChannel::ReliableUnordered,
            NetworkMessageType::WorldInfo => ServerChannel::ReliableOrdered,
        }
    }
}

impl IServerNetwork<RenetServerConnection> for RenetServerNetwork {
    async fn new(ip_port: String) -> Self {
        let server = RenetServer::new(connection_config());

        let socket: UdpSocket = UdpSocket::bind(ip_port.as_str()).unwrap();
        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let server_config = ServerConfig {
            current_time,
            max_clients: 64,
            protocol_id: PROTOCOL_ID,
            public_addresses: vec![socket.local_addr().unwrap()],
            authentication: ServerAuthentication::Unsecure,
        };

        let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
        let network = Self {
            server: Arc::new(RwLock::new(server)),
            transport: Arc::new(RwLock::new(transport)),
            channel_client_messages: flume::unbounded(),
            channel_connections: flume::unbounded(),
            channel_errors: flume::unbounded(),
        };
        network
    }

    async fn step(&self, delta: Duration) {
        let mut server = self.get_server_mut();
        let mut transport = self.get_transport_mut();
        server.update(delta);

        if let Err(e) = transport.update(delta, &mut server) {
            self.channel_errors.0.send(e.to_string()).unwrap();
            return;
        }

        for client_id in server.clients_id().into_iter() {
            for channel_type in ClientChannel::iter() {
                while let Some(client_message) = server.receive_message(client_id, channel_type) {
                    let decoded: ClientMessages = match bincode::deserialize(&client_message) {
                        Ok(d) => d,
                        Err(e) => {
                            log::error!(target: "renet", "Decode client {} message error: {}", channel_type, e);
                            continue;
                        }
                    };
                    // log::info!(target: "network", "server receive message:{}", decoded);
                    self.channel_client_messages.0.send((client_id.raw(), decoded)).unwrap();
                }
            }
        }

        while let Some(event) = server.get_event() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    let addr = transport.client_addr(client_id.clone()).unwrap();
                    let connect = ConnectionMessages::Connect {
                        connection: RenetServerConnection::create(
                            self.server.clone(),
                            client_id.raw(),
                            addr.to_string(),
                        ),
                    };
                    self.channel_connections.0.send(connect).unwrap();
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    let connect = ConnectionMessages::Disconnect {
                        client_id: client_id.raw(),
                        reason: reason.to_string(),
                    };
                    self.channel_connections.0.send(connect).unwrap();
                }
            }
        }

        transport.send_packets(&mut server);
        log::trace!(target: "network", "network step (executed:{:.2?})", delta);
    }

    fn drain_client_messages(&self) -> impl Iterator<Item = (u64, ClientMessages)> {
        self.channel_client_messages.1.drain()
    }

    fn drain_connections(&self) -> impl Iterator<Item = ConnectionMessages<RenetServerConnection>> {
        self.channel_connections.1.drain()
    }

    fn drain_errors(&self) -> impl Iterator<Item = String> {
        self.channel_errors.1.drain()
    }

    fn is_connected(&self, connection: &RenetServerConnection) -> bool {
        self.get_server()
            .is_connected(ClientId::from_raw(connection.get_client_id()))
    }
}

#[derive(Clone)]
pub struct RenetServerConnection {
    server: ServerLock,
    client_id: u64,
    ip: String,
}

impl RenetServerConnection {
    fn create(server: ServerLock, client_id: u64, ip: String) -> Self {
        Self { server, client_id, ip }
    }
}

impl IServerConnection for RenetServerConnection {
    fn get_ip(&self) -> &String {
        &self.ip
    }

    fn get_client_id(&self) -> u64 {
        self.client_id
    }

    fn send_message(&self, message_type: NetworkMessageType, message: &ServerMessages) {
        let encoded = bincode::serialize(message).unwrap();
        let mut server = self.server.as_ref().write().expect("poisoned");
        server.send_message(
            ClientId::from_raw(self.client_id),
            RenetServerNetwork::map_type_channel(message_type),
            encoded,
        );
    }
}
