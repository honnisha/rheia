use bincode::Options;
use common::network_messages::{NetworkMessages, ClientLogin};
use renet::{RenetConnectionConfig, RenetServer, ServerAuthentication, ServerConfig, ServerEvent, DefaultChannel};
use std::{
    net::UdpSocket,
    thread,
    time::{Duration, Instant, SystemTime}, collections::HashMap,
};

use crate::console::console_handler::Console;

const PROTOCOL_ID: u64 = 7;

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
    logins: HashMap<u64, String>,
}

impl NetworkServer {
    fn get_login(&self, client_id: u64) -> &String {
        &self.logins[&client_id]
    }

    pub fn init(ip_port: String) -> Self {
        Console::send_message(format!("Start network server for {}", ip_port));
        NetworkServer {
            server: get_network_server(ip_port),
            last_updated: Instant::now(),
            logins: HashMap::new(),
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
                ServerEvent::ClientConnected(client_id, user_data) => {
                    let login = ClientLogin::from_user_data(&user_data).0;
                    self.logins.insert(client_id, login);
                    Console::send_message(format!("Client \"{}\" connected", self.get_login(client_id)));
                }
                ServerEvent::ClientDisconnected(client_id) => {
                    Console::send_message(format!("Client \"{}\" disconnected", self.get_login(client_id)));
                }
            }
        }

        // Receive message from channel
        for client_id in self.server.clients_id().into_iter() {
            while let Some(message) = self.server.receive_message(client_id, DefaultChannel::Reliable) {
                // Handle received message
                let data: NetworkMessages = bincode::options().deserialize(&message).unwrap();
                println!("data: {:?}", data);
                match data {
                    NetworkMessages::ConsoleCommand { command } => {
                        Console::send_message(format!("Console sended {}: {}", self.get_login(client_id), command));
                    },
                    _ => {},
                }
            }
        }

        // Send a text message for all clients
        self.server
            .broadcast_message(DefaultChannel::Reliable, "server message".as_bytes().to_vec());

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
