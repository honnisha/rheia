use crate::main_scene::Main;
use bincode::Options;
use common::network::ClientChannel;
use common::network::ClientMessages;
use common::network::PROTOCOL_ID;
use common::network::ServerMessages;
use godot::engine::Engine;
use lazy_static::lazy_static;
use log::info;
use renet::transport::ClientAuthentication;
use renet::transport::NetcodeClientTransport;
use renet::DefaultChannel;
use renet::{ConnectionConfig, RenetClient};
use std::net::UdpSocket;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;
use std::time::SystemTime;

lazy_static! {
    static ref NETWORK_CONTAINER: Arc<RwLock<NetworkContainer>> = Arc::new(RwLock::new(NetworkContainer::new()));
}

pub struct NetworkContainer {
    pub client: Option<Arc<RwLock<RenetClient>>>,
    pub transport: Option<Arc<RwLock<NetcodeClientTransport>>>,
}

impl NetworkContainer {
    pub fn new() -> Self {
        NetworkContainer {
            client: None,
            transport: None,
        }
    }

    pub fn create_client(ip_port: String) {
        info!("Connecting to the server at {}", ip_port);
        let mut network_handler = NETWORK_CONTAINER.write().unwrap();

        network_handler.client = Some(Arc::new(RwLock::new(RenetClient::new(ConnectionConfig::default()))));

        // Setup transport layer
        // const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1), 5000));
        let server_addr = ip_port.clone().parse().unwrap();
        let socket = match UdpSocket::bind("127.0.0.1:0") {
            Ok(s) => s,
            Err(e) => {
                info!("IP {} error: {}", ip_port, e);
                return;
            },
        };
        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let client_id: u64 = 0;
        let authentication = ClientAuthentication::Unsecure {
            server_addr: server_addr,
            client_id,
            user_data: None,
            protocol_id: PROTOCOL_ID,
        };

        network_handler.transport = Some(Arc::new(RwLock::new(
            NetcodeClientTransport::new(current_time, authentication, socket).unwrap(),
        )));
    }

    pub fn update(delta: f64, _main_scene: &mut Main) {
        let delta_time = Duration::from_secs_f64(delta);
        let container = NETWORK_CONTAINER.read().unwrap();

        let mut client = container.client.as_ref().unwrap().write().unwrap();
        let mut transport = container.transport.as_ref().unwrap().write().unwrap();

        client.update(delta_time);
        transport.update(delta_time, &mut client).unwrap();

        if !client.is_disconnected() {
            while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
                let message: ServerMessages = bincode::options().deserialize(&message).unwrap();
                match message {
                    ServerMessages::ConsoleOutput { command } => {
                        info!("{}", command);
                    }
                }
            }
        }

        transport.send_packets(&mut client).unwrap();
    }

    pub fn disconnect() {
        let container = NETWORK_CONTAINER.read().unwrap();

        let mut client = container.client.as_ref().unwrap().write().unwrap();
        client.disconnect();
    }

    pub fn send_console_command(command: String) {
        let container = NETWORK_CONTAINER.read().unwrap();

        let mut client = container.client.as_ref().unwrap().write().unwrap();
        let input = ClientMessages::ConsoleInput { command: command };
        let command_message = bincode::serialize(&input).unwrap();
        client.send_message(ClientChannel::ClientMessages, command_message);
    }
}
