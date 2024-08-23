use crate::network::{
    messages::{ClientMessages, NetworkMessageType, ServerMessages},
    server::{ConnectionMessages, ServerNetwork},
};
use ahash::AHashMap;
use networking_sockets::{ListenSocket, NetConnection};
use networking_types::{NetConnectionEnd, NetworkingConfigEntry, SendFlags};
use parking_lot::RwLock;
use std::sync::Arc;
use steamworks::*;

pub struct SteamworksServer<Manager = ClientManager> {
    single: SingleClient,
    listen_socket: ListenSocket<Manager>,
    matchmaking: Matchmaking<Manager>,
    friends: Friends<Manager>,
    connections: Arc<RwLock<AHashMap<u64, NetConnection<Manager>>>>,

    channel_client_messages: (
        flume::Sender<(u64, ClientMessages)>,
        flume::Receiver<(u64, ClientMessages)>,
    ),
    channel_connections: (flume::Sender<ConnectionMessages>, flume::Receiver<ConnectionMessages>),
    channel_errors: (flume::Sender<String>, flume::Receiver<String>),
}

const MAX_MESSAGE_BATCH_SIZE: usize = 512;

// https://github.com/lucaspoffo/renet/blob/master/renet_steam/src/server.rs

impl SteamworksServer {
    fn map_type_channel(message_type: NetworkMessageType) -> SendFlags {
        match message_type {
            NetworkMessageType::ReliableOrdered => SendFlags::RELIABLE,
            NetworkMessageType::Unreliable => SendFlags::UNRELIABLE,
            NetworkMessageType::ReliableUnordered => SendFlags::RELIABLE,
        }
    }
}

impl ServerNetwork for SteamworksServer {
    fn new(_ip_port: String) -> Self {
        let (client, single) = Client::init_app(480).unwrap();
        client.networking_utils().init_relay_network_access();

        let options: Vec<NetworkingConfigEntry> = Vec::new();
        let listen_socket = client
            .networking_sockets()
            .create_listen_socket_p2p(0, options)
            .unwrap();
        Self {
            single,
            listen_socket,
            matchmaking: client.matchmaking(),
            friends: client.friends(),
            connections: Arc::new(RwLock::new(AHashMap::new())),
            channel_client_messages: flume::unbounded(),
            channel_connections: flume::unbounded(),
            channel_errors: flume::unbounded(),
        }
    }

    fn step(&self, _delta: std::time::Duration) -> bool {
        while let Some(event) = self.listen_socket.try_receive_event() {
            match event {
                networking_types::ListenSocketEvent::Connecting(event) => {
                    let Some(steam_id) = event.remote().steam_id() else {
                        event.reject(NetConnectionEnd::AppGeneric, Some("Invalid steam id"));
                        continue;
                    };
                    if let Err(e) = event.accept() {
                        log::error!(target: "steamworks", "Failed to accept connection from {steam_id:?}: {e}");
                    }
                }
                networking_types::ListenSocketEvent::Connected(event) => {
                    if let Some(steam_id) = event.remote().steam_id() {
                        self.connections.write().insert(steam_id.raw(), event.take_connection());
                        let connect = ConnectionMessages::Connect {
                            client_id: steam_id.raw(),
                            ip: steam_id.raw().to_string(),
                        };
                        self.channel_connections.0.send(connect).unwrap();
                    }
                }
                networking_types::ListenSocketEvent::Disconnected(event) => {
                    if let Some(steam_id) = event.remote().steam_id() {
                        let connect = ConnectionMessages::Disconnect {
                            client_id: steam_id.raw(),
                            reason: (event.end_reason() as i32).to_string(),
                        };
                        self.channel_connections.0.send(connect).unwrap();
                    }
                }
            }
        }
        for (client_id, connection) in self.connections.write().iter_mut() {
            // TODO this allocates on the side of steamworks.rs and should be avoided, PR needed
            if let Ok(messages) = connection.receive_messages(MAX_MESSAGE_BATCH_SIZE) {
                for message in messages.iter() {
                    let decoded: ClientMessages = match bincode::deserialize(&message.data()) {
                        Ok(d) => d,
                        Err(e) => {
                            log::error!(target: "steamworks", "Decode client unreliable message error: {}", e);
                            continue;
                        }
                    };
                    self.channel_client_messages
                        .0
                        .send((client_id.clone(), decoded))
                        .unwrap();
                }
            }
        }
        return true;
    }

    fn drain_client_messages(&self) -> impl Iterator<Item = (u64, ClientMessages)> {
        self.channel_client_messages.1.drain()
    }

    fn drain_connections(&self) -> impl Iterator<Item = ConnectionMessages> {
        self.channel_connections.1.drain()
    }

    fn drain_errors(&self) -> impl Iterator<Item = String> {
        self.channel_errors.1.drain()
    }

    fn is_connected(&self, client_id: u64) -> bool {
        self.connections.read().contains_key(&client_id)
    }

    fn send_message(&self, client_id: u64, message: &ServerMessages, message_type: NetworkMessageType) {
        let connections = self.connections.read();
        let Some(connection) = connections.get(&client_id) else {
            panic!("Error while sending packet: connection not found");
        };
        let encoded = bincode::serialize(message).unwrap();
        if let Err(e) = connection.send_message(&encoded, SteamworksServer::map_type_channel(message_type)) {
            panic!("Failed to send packet to client {client_id}: {e}");
        }
        if let Err(e) = connection.flush_messages() {
            panic!("Failed flush messages for {client_id}: {e}");
        }
    }
}
