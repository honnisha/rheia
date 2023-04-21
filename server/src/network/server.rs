use renet::{RenetConnectionConfig, RenetServer, ServerAuthentication, ServerConfig, ServerEvent};
use std::{
    net::UdpSocket,
    thread,
    time::{Duration, Instant, SystemTime},
};

use crate::console::console_handler::Console;

const PROTOCOL_ID: u64 = 7;
pub const CHANNEL_ID: u8 = 0;

fn get_network_server(ip_port: String) -> RenetServer {
    //let socket = UdpSocket::bind(ip_port.clone()).unwrap();
    //let server_addr = socket.local_addr().unwrap();
    let server_addr = ip_port.parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();

    let server_config = ServerConfig::new(64, PROTOCOL_ID, server_addr, ServerAuthentication::Unsecure);
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let connection_config = RenetConnectionConfig::default();
    RenetServer::new(current_time, server_config, connection_config, socket).unwrap()
}

pub struct NetworkServer {
    server: RenetServer,
    last_updated: Instant,
}

impl NetworkServer {
    pub fn init(ip_port: String) -> Self {
        Console::send_message(format!("Start network server for {}", ip_port));
        NetworkServer {
            server: get_network_server(ip_port),
            last_updated: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        // Receive new messages and update clients
        let now = Instant::now();
        self.server.update(now - self.last_updated).unwrap();
        self.last_updated = now;

        // Check for client connections/disconnections
        while let Some(event) = self.server.get_event() {
            match event {
                ServerEvent::ClientConnected(id, user_data) => {
                    Console::send_message(format!("Client {} connected", id));
                }
                ServerEvent::ClientDisconnected(id) => {
                    Console::send_message(format!("Client {} disconnected", id));
                }
            }
        }

        // Receive message from channel
        for client_id in self.server.clients_id().into_iter() {
            while let Some(message) = self.server.receive_message(client_id, CHANNEL_ID) {
                // Handle received message
            }
        }

        // Send a text message for all clients
        self.server
            .broadcast_message(CHANNEL_ID, "server message".as_bytes().to_vec());

        // Send message to only one client
        //let client_id = ...;
        //server.send_message(client_id, channel_id, "server message".as_bytes().to_vec());

        // Send packets to clients
        self.server.send_packets().unwrap();
        thread::sleep(Duration::from_millis(50));
    }

    pub fn stop(&mut self) {
        println!("Stopping the server");
    }
}
