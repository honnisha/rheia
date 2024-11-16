use flume::{Drain, Receiver, Sender};
use parking_lot::RwLockReadGuard;
use parking_lot::{RwLock, RwLockWriteGuard};
use renet::{
    transport::{ClientAuthentication, NetcodeClientTransport},
    Bytes, RenetClient,
};
use rhai::Instant;
use std::thread;
use std::{
    net::UdpSocket,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, SystemTime},
};

use crate::client::{resolve_connect_domain, IClientNetwork};
use crate::messages::ClientMessages;
use crate::messages::NetworkMessageType;
use crate::{client::NetworkInfo, messages::ServerMessages};

use super::channels::ServerChannel;
use super::{connection_config, PROTOCOL_ID};

type ClientLock = Arc<RwLock<RenetClient>>;
type TransferLock = Arc<RwLock<NetcodeClientTransport>>;

type ClientMessageType = (u8, Vec<u8>);

#[derive(Clone)]
pub struct RenetClientNetwork {
    client: ClientLock,
    transport: TransferLock,

    timer: Arc<RwLock<Instant>>,
    network_info: Arc<RwLock<NetworkInfo>>,

    network_decoder_out: (Sender<ServerMessages>, Receiver<ServerMessages>),
    network_errors_out: (Sender<String>, Receiver<String>),
    network_lock: Arc<AtomicBool>,

    // Messages was sended by the client
    // must be sended to the server
    network_client_sended: (Sender<ClientMessageType>, Receiver<ClientMessageType>),
}

impl RenetClientNetwork {
    pub fn is_network_locked(&self) -> bool {
        self.network_lock.load(Ordering::Relaxed)
    }

    pub fn set_network_lock(&self, state: bool) {
        self.network_lock.store(state, Ordering::Relaxed);
    }

    fn get_client_mut(&self) -> RwLockWriteGuard<RenetClient> {
        self.client.write()
    }

    fn get_transport(&self) -> RwLockReadGuard<NetcodeClientTransport> {
        self.transport.read()
    }

    fn get_transport_mut(&self) -> RwLockWriteGuard<NetcodeClientTransport> {
        self.transport.write()
    }

    fn get_delta_time(&self) -> Duration {
        let mut t = self.timer.write();
        let delta_time = t.elapsed();
        *t = Instant::now();
        delta_time
    }

    /// Send decoded message to thread server messages channel
    fn send_server_message(&self, decoded: ServerMessages) {
        self.network_decoder_out.0.send(decoded).unwrap();
    }

    /// Send error message to thread server messages channel
    fn send_network_error(&self, message: String) {
        self.network_errors_out.0.send(message).unwrap();
    }

    fn decode_server_message(encoded: &Bytes) -> Option<ServerMessages> {
        let decoded: ServerMessages = match bincode::deserialize(encoded) {
            Ok(d) => d,
            Err(e) => {
                log::error!(target: "renet", "Decode server heavy message error: {}", e);
                return None;
            }
        };
        Some(decoded)
    }

    fn map_type_channel(message_type: NetworkMessageType) -> ServerChannel {
        match message_type {
            NetworkMessageType::ReliableOrdered => ServerChannel::ReliableOrdered,
            NetworkMessageType::ReliableUnordered => ServerChannel::ReliableUnordered,
            NetworkMessageType::Unreliable => ServerChannel::Unreliable,
            NetworkMessageType::WorldInfo => ServerChannel::ReliableOrdered,
        }
    }

    fn step(&self, delta: std::time::Duration) -> bool {
        let mut client = self.get_client_mut();

        if client.is_disconnected() {
            return false;
        }

        {
            let info = client.network_info();
            let mut network_info = self.network_info.write();
            network_info.is_disconnected = client.is_disconnected();
            network_info.bytes_received_per_second = info.bytes_received_per_second;
            network_info.bytes_received_per_sec = client.bytes_received_per_sec();
            network_info.bytes_sent_per_sec = client.bytes_sent_per_sec();
            network_info.packet_loss = client.packet_loss();
        }

        client.update(delta);
        let mut transport = self.get_transport_mut();
        if let Err(e) = transport.update(delta, &mut client) {
            self.send_network_error(e.to_string());
            return false;
        }

        while let Some(server_message) = client.receive_message(ServerChannel::ReliableOrdered) {
            let decoded = RenetClientNetwork::decode_server_message(&server_message);
            if let Some(d) = decoded {
                self.send_server_message(d);
            }
        }
        while let Some(server_message) = client.receive_message(ServerChannel::ReliableUnordered) {
            let decoded = RenetClientNetwork::decode_server_message(&server_message);
            if let Some(d) = decoded {
                self.send_server_message(d);
            }
        }
        while let Some(server_message) = client.receive_message(ServerChannel::Unreliable) {
            let decoded = RenetClientNetwork::decode_server_message(&server_message);
            if let Some(d) = decoded {
                self.send_server_message(d);
            }
        }

        if client.is_connected() {
            for (channel, message) in self.network_client_sended.1.drain() {
                client.send_message(channel, message);
            }
        }

        if let Err(e) = transport.send_packets(&mut client) {
            self.send_network_error(e.to_string());
            return false;
        }
        return true;
    }

    fn spawn_network_thread(&self) {
        let container = self.clone();
        thread::spawn(move || loop {
            {
                // Network will be processed only when there is no lock
                if container.is_network_locked() {
                    continue;
                }
                container.set_network_lock(true);

                let success = container.step(container.get_delta_time());
                if !success {
                    break;
                }
            }
            thread::sleep(Duration::from_millis(10));
        });
        log::info!(target: "renet", "Network thread spawned");
    }
}

impl IClientNetwork for RenetClientNetwork {
    fn new(ip_port: String) -> Result<Self, String> {
        let client = RenetClient::new(connection_config());

        // Setup transport layer
        let server_addr = match resolve_connect_domain(&ip_port, 25565_u16) {
            Ok(a) => a,
            Err(e) => return Err(format!("Path {} error: {}", ip_port, e)),
        };

        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let client_id = current_time.as_millis() as u64;
        let authentication = ClientAuthentication::Unsecure {
            server_addr: server_addr,
            client_id,
            user_data: None,
            protocol_id: PROTOCOL_ID,
        };

        let socket = match UdpSocket::bind("0.0.0.0:0") {
            Ok(s) => s,
            Err(e) => {
                return Err(format!("Path {} error: {}", ip_port, e));
            }
        };
        let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
        let network = Self {
            client: Arc::new(RwLock::new(client)),
            transport: Arc::new(RwLock::new(transport)),

            network_info: Arc::new(RwLock::new(Default::default())),
            timer: Arc::new(RwLock::new(Instant::now())),
            network_decoder_out: flume::unbounded(),
            network_errors_out: flume::unbounded(),
            network_lock: Arc::new(AtomicBool::new(false)),
            network_client_sended: flume::unbounded(),
        };
        network.spawn_network_thread();
        Ok(network)
    }

    fn iter_server_messages(&self) -> Drain<ServerMessages> {
        let drain = self.network_decoder_out.1.drain();
        self.set_network_lock(false);
        drain
    }

    fn iter_errors(&self) -> Drain<String> {
        self.network_errors_out.1.drain()
    }

    fn is_connected(&self) -> bool {
        self.get_transport().disconnect_reason().is_none()
    }

    fn send_message(&self, message_type: NetworkMessageType, message: &ClientMessages) {
        let encoded = bincode::serialize(message).unwrap();
        let msg = (RenetClientNetwork::map_type_channel(message_type).into(), encoded);
        self.network_client_sended.0.send(msg).unwrap();
    }

    fn disconnect(&self) {
        let mut transport = self.get_transport_mut();
        if transport.disconnect_reason().is_none() {
            transport.disconnect();
            log::info!(target: "renet", "{}", "Disconnected from the server");
        }
    }

    fn get_network_info(&self) -> RwLockReadGuard<NetworkInfo> {
        self.network_info.read()
    }
}
