use crate::controller::player_movement::PlayerMovement;
use common::chunks::utils::unpack_network_sectioins;
use common::network::channels::ClientChannel;
use common::network::channels::ServerChannel;
use common::network::connection_config;
use common::network::login::Login;
use common::network::messages::ClientMessages;
use common::network::messages::ServerMessages;
use common::network::PROTOCOL_ID;
use flume::Drain;
use flume::{unbounded, Receiver, Sender};
use lazy_static::lazy_static;
use log::error;
use log::info;
use renet::transport::ClientAuthentication;
use renet::transport::NetcodeClientTransport;
use renet::Bytes;
use renet::RenetClient;
use std::net::UdpSocket;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;
use std::time::Duration;
use std::time::Instant;
use std::time::SystemTime;
use std::{thread, time};

lazy_static! {
    static ref NETWORK_CONTAINER: Arc<RwLock<Option<NetworkContainer>>> = Arc::new(RwLock::new(None));
    static ref NETWORK_DECODER_OUT: (Sender<ServerMessages>, Receiver<ServerMessages>) = unbounded();
    static ref NETWORK_ERRORS_OUT: (Sender<String>, Receiver<String>) = unbounded();
    static ref NETWORK_LOCK: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
}

pub struct NetworkContainer {
    client: Arc<RwLock<RenetClient>>,
    transport: Arc<RwLock<NetcodeClientTransport>>,
    timer: Arc<RwLock<Instant>>,
    pub network_info: Arc<RwLock<NetworkInfo>>,
}

#[derive(Default)]
pub struct NetworkInfo {
    pub is_disconnected: bool,
    pub bytes_received_per_second: f64,
    pub bytes_received_per_sec: f64,
    pub bytes_sent_per_sec: f64,
    pub packet_loss: f64,
}

impl NetworkContainer {
    pub fn new(client: RenetClient, transport: NetcodeClientTransport) -> Self {
        Self {
            client: Arc::new(RwLock::new(client)),
            transport: Arc::new(RwLock::new(transport)),
            timer: Arc::new(RwLock::new(Instant::now())),
            network_info: Arc::new(RwLock::new(Default::default())),
        }
    }

    pub fn create_client(ip_port: String, login: String) -> Result<(), String> {
        info!("Connecting to the server at {}", ip_port);
        let client = RenetClient::new(connection_config());

        // Setup transport layer
        let server_addr = ip_port.clone().parse().unwrap();
        let socket = match UdpSocket::bind("127.0.0.1:0") {
            Ok(s) => s,
            Err(e) => {
                return Err(format!("IP {} error: {}", ip_port, e));
            }
        };
        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let client_id = current_time.as_millis() as u64;
        let authentication = ClientAuthentication::Unsecure {
            server_addr: server_addr,
            client_id,
            user_data: Some(Login(login).to_netcode_user_data()),
            protocol_id: PROTOCOL_ID,
        };

        let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

        let mut network_handler = NETWORK_CONTAINER.write().unwrap();
        *network_handler = Some(NetworkContainer::new(client, transport));

        Ok(())
    }

    pub fn is_network_locked() -> bool {
        NETWORK_LOCK.load(Ordering::Relaxed)
    }

    pub fn set_network_lock(state: bool) {
        NETWORK_LOCK.store(state, Ordering::Relaxed);
    }

    pub fn read() -> RwLockReadGuard<'static, Option<NetworkContainer>> {
        NETWORK_CONTAINER.read().unwrap()
    }

    pub fn has_client() -> bool {
        NETWORK_CONTAINER.read().unwrap().is_some()
    }

    fn get_client_mut(&self) -> RwLockWriteGuard<RenetClient> {
        self.client.write().expect("poisoned")
    }

    fn get_transport_mut(&self) -> RwLockWriteGuard<NetcodeClientTransport> {
        self.transport.write().expect("poisoned")
    }

    fn get_delta_time(&self) -> Duration {
        let mut t = self.timer.write().unwrap();
        let delta_time = t.elapsed();
        *t = Instant::now();
        delta_time
    }

    /// Spawns network thread
    /// which is recieve network messages, decode and send them
    /// to the channel
    pub fn spawn_network_thread() {
        thread::spawn(move || loop {
            {
                // Network will be processed only when there is no lock
                if NetworkContainer::is_network_locked() {
                    continue;
                }
                NetworkContainer::set_network_lock(true);

                let success = NetworkContainer::step();
                if !success {
                    break;
                }
            }
            thread::sleep(time::Duration::from_millis(50));
        });
        info!("Network thread spawned");
    }

    fn step() -> bool {
        let c = NetworkContainer::read();
        let container = c.as_ref().unwrap();

        let mut client = container.get_client_mut();

        {
            let info = client.network_info();
            let mut network_info = container.network_info.write().unwrap();
            network_info.is_disconnected = client.is_disconnected();
            network_info.bytes_received_per_second = info.bytes_received_per_second;
            network_info.bytes_received_per_sec = client.bytes_received_per_sec();
            network_info.bytes_sent_per_sec = client.bytes_sent_per_sec();
            network_info.packet_loss = client.packet_loss();
        }

        if client.is_disconnected() {
            return false;
        }

        let delta_time = container.get_delta_time();
        client.update(delta_time);
        let mut transport = container.get_transport_mut();
        if let Err(e) = transport.update(delta_time, &mut client) {
            NetworkContainer::send_network_error(e.to_string());
            return false;
        }

        while let Some(server_message) = client.receive_message(ServerChannel::Reliable) {
            let decoded = NetworkContainer::decode_server_message(&server_message);
            if let Some(d) = decoded {
                NetworkContainer::send_server_message(d);
            }
        }
        while let Some(server_message) = client.receive_message(ServerChannel::Chunks) {
            let decoded = NetworkContainer::decode_server_message(&server_message);
            if let Some(d) = decoded {
                match d {
                    ServerMessages::ChunkSectionInfo {
                        world_slug: _,
                        chunk_position,
                        sections: _,
                    } => {
                        let input = ClientMessages::ChunkRecieved {
                            chunk_position: chunk_position,
                        };
                        let chunk_recieved = bincode::serialize(&input).unwrap();
                        client.send_message(ClientChannel::Reliable, chunk_recieved);
                    }
                    _ => (),
                }
                NetworkContainer::send_server_message(d);
            }
        }
        if let Err(e) = transport.send_packets(&mut client) {
            NetworkContainer::send_network_error(e.to_string());
            return false;
        }
        return true;
    }

    fn decode_server_message(encoded: &Bytes) -> Option<ServerMessages> {
        let decoded: ServerMessages = match bincode::deserialize(encoded) {
            Ok(d) => d,
            Err(e) => {
                error!("Decode server heavy message error: {}", e);
                return None;
            }
        };
        // Handle decoded messages before send them back
        let decoded: ServerMessages = match decoded {
            ServerMessages::ChunkSectionEncodedInfo {
                world_slug,
                chunk_position,
                mut sections,
            } => ServerMessages::ChunkSectionInfo {
                world_slug,
                chunk_position,
                sections: unpack_network_sectioins(&mut sections),
            },
            other => other,
        };

        Some(decoded)
    }

    /// Send decoded message to thread server messages channel
    fn send_server_message(decoded: ServerMessages) {
        NETWORK_DECODER_OUT.0.send(decoded).unwrap();
    }

    /// Send error message to thread server messages channel
    fn send_network_error(message: String) {
        NETWORK_ERRORS_OUT.0.send(message).unwrap();
    }

    pub fn errors_iter() -> Drain<'static, String> {
        NETWORK_ERRORS_OUT.1.drain()
    }

    pub fn server_messages_iter() -> Drain<'static, ServerMessages> {
        // Remove the restriction so that the network
        // can continue receiving messages
        NetworkContainer::set_network_lock(false);

        NETWORK_DECODER_OUT.1.drain()
    }

    pub fn disconnect() {
        let c = NetworkContainer::read();
        let container = c.as_ref().unwrap();

        let mut transport = container.get_transport_mut();
        if transport.is_connected() {
            transport.disconnect();
            info!("{}", "Disconnected from the server");
        }
    }

    pub fn send_console_command(command: String) {
        let c = NetworkContainer::read();
        let container = c.as_ref().unwrap();

        let mut client = container.get_client_mut();
        let input = ClientMessages::ConsoleInput { command: command };
        let command_message = bincode::serialize(&input).unwrap();
        client.send_message(ClientChannel::Reliable, command_message);
    }

    pub fn send_player_move(movement: PlayerMovement) {
        let c = NetworkContainer::read();
        let container = c.as_ref().unwrap();

        let mut client = container.get_client_mut();
        let message = bincode::serialize(&movement.into_network()).unwrap();
        client.send_message(ClientChannel::Unreliable, message);
    }
}
