use std::{
    net::ToSocketAddrs,
    sync::{Arc, Mutex},
};
use log::info;
use crate::main_scene::Main;
use bincode::DefaultOptions;
use lazy_static::lazy_static;
use network::packet_length_serializer::LittleEndian;
use network::{
    client::ClientNetwork, protocols::tcp::TcpProtocol, serializers::bincode::BincodeSerializer, ClientConfig,
    ClientPacket, ServerPacket,
};

lazy_static! {
    static ref NETWORK_CONTAINER: Arc<Mutex<NetworkContainer>> = Arc::new(Mutex::new(NetworkContainer::new()));
}

struct Config;

impl ClientConfig for Config {
    type ClientPacket = ClientPacket;
    type ServerPacket = ServerPacket;
    type Protocol = TcpProtocol;
    type Serializer = BincodeSerializer<DefaultOptions>;
    type LengthSerializer = LittleEndian<u32>;
}

pub struct NetworkContainer {
    client: ClientNetwork<Config>,
}

impl NetworkContainer {
    pub fn new() -> Self {
        NetworkContainer {
            client: ClientNetwork::<Config>::init(),
        }
    }

    pub fn create_client(ip_port: String) {
        info!("Connecting to the server at {}", ip_port);
        let address = ip_port
            .to_socket_addrs()
            .expect("Invalid address")
            .next()
            .expect("Invalid address");

        let mut network_handler = NETWORK_CONTAINER.lock().unwrap();

        network_handler.client.connect(address);
    }

    pub fn update(_delta: f64, _main_scene: &mut Main) {
        let mut container = NETWORK_CONTAINER.lock().unwrap();

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
        while let Ok((_connection, packet)) = container.client.packet_receiver_rx.try_recv() {}
    }

    pub fn disconnect() {}

    pub fn send_console_command(_command: String) {}
}
