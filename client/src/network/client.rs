use crate::main_scene::Main;
use bincode::DefaultOptions;
use lazy_static::lazy_static;
use log::info;
use network::packet_length_serializer::LittleEndian;
use network::{
    client::ClientNetwork, protocols::tcp::TcpProtocol, serializers::bincode::BincodeSerializer, ClientConfig,
    ClientPacket, ServerPacket,
};
use std::sync::RwLock;
use std::{net::ToSocketAddrs, sync::Arc, time::Duration};

lazy_static! {
    static ref NETWORK_CONTAINER: Arc<RwLock<NetworkContainer>> = Arc::new(RwLock::new(NetworkContainer::new()));
}

pub(crate) struct Config;

impl ClientConfig for Config {
    type ClientPacket = ClientPacket;
    type ServerPacket = ServerPacket;
    type Protocol = TcpProtocol;
    type Serializer = BincodeSerializer<DefaultOptions>;
    type LengthSerializer = LittleEndian<u32>;
}

pub struct NetworkContainer {
    client: ClientNetwork<Config>,

    keepalive_delay: Duration,
    keepalive_runtime_timer: Duration,
    keepalive_server_limit: Duration,
    keepalive_server_timer: Duration,
}

impl NetworkContainer {
    pub fn new() -> Self {
        NetworkContainer {
            client: ClientNetwork::<Config>::init(),
            keepalive_delay: Duration::from_secs_f32(0.5),
            keepalive_runtime_timer: Duration::from_secs_f32(0.0),
            keepalive_server_limit: Duration::from_secs_f32(5.0),
            keepalive_server_timer: Duration::from_secs_f32(0.0),
        }
    }

    pub fn create_client(ip_port: String) {
        info!("Connecting to the server at {}", ip_port);
        let address = ip_port
            .to_socket_addrs()
            .expect("Invalid address")
            .next()
            .expect("Invalid address");

        let mut network_handler = NETWORK_CONTAINER.write().unwrap();

        network_handler.client.connect(address);
    }

    pub fn update(delta: f64, _main_scene: &mut Main) {
        let mut container = NETWORK_CONTAINER.write().unwrap();

        if container.client.connections.has_connection() {
            // Keep alive
            container.keepalive_runtime_timer += Duration::from_secs_f64(delta);
            if container.keepalive_runtime_timer >= container.keepalive_delay {
                let connection = container.client.connections.get_connection().unwrap();
                connection.send(ClientPacket::KeepAlive).unwrap();
                container.keepalive_runtime_timer = Duration::from_secs_f32(0.0);
            }

            // Check timeout from server
            container.keepalive_server_timer += Duration::from_secs_f64(delta);
            if container.keepalive_server_timer >= container.keepalive_server_limit {
                info!("Disconnected: time out");
            }
        }

        // connection_establish_system
        while let Ok((_address, connection)) = container.client.connection_receiver_rx.try_recv() {
            container.client.connections.push(connection.clone());
            info!("Connected successfully");
        }

        // connection_remove_system
        while let Ok((error, address)) = container.client.disconnection_receiver_rx.try_recv() {
            container.client.connections.retain(|conn| conn.peer_addr() != address);
            info!("Disconnected: {:?}", error);
        }

        // packet_receive_system
        while let Ok((_connection, packet)) = container.client.packet_receiver_rx.try_recv() {
            match packet {
                ServerPacket::KeepAlive => container.keepalive_server_timer = Duration::from_secs_f32(0.0),
                ServerPacket::ConsoleOutput(message) => info!("{}", message),
            }
        }
    }

    pub fn disconnect() {
        let mut container = NETWORK_CONTAINER.write().unwrap();
        if let Some(c) = container.client.connections.get_connection() {
            c.disconnect();
        }
    }

    pub fn send_console_command(command: String) {
        let mut container = NETWORK_CONTAINER.write().unwrap();
        if let Some(c) = container.client.connections.get_connection() {
            c.send(ClientPacket::ConsoleInput(command)).unwrap();
        }
    }
}
