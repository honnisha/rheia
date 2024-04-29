use flume::{Drain, Receiver, Sender};
use std::{
    net::{UdpSocket, SocketAddr},
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
    time::{Duration, SystemTime},
};

use renet::{
    transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
    RenetServer, ServerEvent, ClientId,
};

use super::{
    channels::{ClientChannel, ServerChannel},
    connection_config, PROTOCOL_ID,
};
use crate::network::{
    messages::{ClientMessages, NetworkMessageType, ServerMessages},
    server::{ConnectionMessages, ServerNetwork},
};
use log::error;

type ServerLock = Arc<RwLock<RenetServer>>;
type TransferLock = Arc<RwLock<NetcodeServerTransport>>;

pub struct RenetServerNetwork {
    server: ServerLock,
    transport: TransferLock,
    channel_client_messages: (Sender<(u64, ClientMessages)>, Receiver<(u64, ClientMessages)>),
    channel_connections: (Sender<ConnectionMessages>, Receiver<ConnectionMessages>),
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
        }
    }
}

impl ServerNetwork for RenetServerNetwork {
    fn new(ip_port: String) -> Self {
        let server = RenetServer::new(connection_config());

        let server_addr: SocketAddr = ip_port.parse().unwrap();
        let socket: UdpSocket = UdpSocket::bind(server_addr).unwrap();

        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let server_config = ServerConfig {
            current_time,
            max_clients: 64,
            protocol_id: PROTOCOL_ID,
            public_addresses: vec![server_addr],
            authentication: ServerAuthentication::Unsecure,
        };

        let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
        let network = RenetServerNetwork {
            server: Arc::new(RwLock::new(server)),
            transport: Arc::new(RwLock::new(transport)),
            channel_client_messages: flume::unbounded(),
            channel_connections: flume::unbounded(),
            channel_errors: flume::unbounded(),
        };
        network
    }

    fn step(&self, delta: Duration) -> bool {
        let mut server = self.get_server_mut();
        let mut transport = self.get_transport_mut();
        server.update(delta);

        if let Err(e) = transport.update(delta, &mut server) {
            self.channel_errors.0.send(e.to_string()).unwrap();
        }

        for client_id in server.clients_id().into_iter() {
            while let Some(client_message) = server.receive_message(client_id, ClientChannel::ReliableOrdered) {
                let decoded: ClientMessages = match bincode::deserialize(&client_message) {
                    Ok(d) => d,
                    Err(e) => {
                        error!("Decode client reliable message error: {}", e);
                        continue;
                    }
                };
                self.channel_client_messages
                    .0
                    .send((client_id.raw(), decoded))
                    .unwrap();
            }
            while let Some(client_message) = server.receive_message(client_id, ClientChannel::Unreliable) {
                let decoded: ClientMessages = match bincode::deserialize(&client_message) {
                    Ok(d) => d,
                    Err(e) => {
                        error!("Decode client unreliable message error: {}", e);
                        continue;
                    }
                };
                self.channel_client_messages
                    .0
                    .send((client_id.raw(), decoded))
                    .unwrap();
            }
        }

        while let Some(event) = server.get_event() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    let addr = transport.client_addr(client_id.clone()).unwrap();
                    let connect = ConnectionMessages::Connect {
                        client_id: client_id.raw(),
                        ip: addr.to_string(),
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
            //server_events.send(event);
        }

        transport.send_packets(&mut server);
        return true;
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
        self.get_server().is_connected(ClientId::from_raw(client_id))
    }

    fn send_message(&self, client_id: u64, message: &ServerMessages, message_type: NetworkMessageType) {
        let encoded = bincode::serialize(message).unwrap();
        self.get_server_mut()
            .send_message(ClientId::from_raw(client_id), RenetServerNetwork::map_type_channel(message_type), encoded);
    }
}
