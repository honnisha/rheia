use bincode::Options;
use common::network_messages::{ClentMessages, ClientLogin};
use lazy_static::lazy_static;
use renet::{DefaultChannel, RenetConnectionConfig, RenetServer, ServerAuthentication, ServerConfig, ServerEvent};
use std::{
    collections::HashMap,
    net::UdpSocket,
    thread,
    time::{Duration, Instant, SystemTime},
};

use super::player::Player;
use crate::console::console_handler::Console;
use crate::CONSOLE_HANDLER;
use crossbeam_channel::{unbounded, Receiver, Sender};

const PROTOCOL_ID: u64 = 7;

fn get_network_server(ip_port: String) -> RenetServer {
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
    players: HashMap<u64, Player>,
}

struct ConsoleOutput {
    client_id: u64,
    message: Vec<u8>,
}

impl ConsoleOutput {
    pub fn init(client_id: u64, message: Vec<u8>) -> Self {
        ConsoleOutput {
            client_id: client_id,
            message: message,
        }
    }
}

unsafe impl Send for ConsoleOutput {}
unsafe impl Sync for ConsoleOutput {}

lazy_static! {
    static ref NETWORK_CONSOLE_OUTPUT: (Sender<ConsoleOutput>, Receiver<ConsoleOutput>) = unbounded();
}

impl NetworkServer {
    fn get_player(&self, client_id: u64) -> &Player {
        &self.players[&client_id]
    }

    pub fn init(ip_port: String) -> Self {
        Console::send_message(format!("Start network server for {}", ip_port));
        NetworkServer {
            server: get_network_server(ip_port),
            last_updated: Instant::now(),
            players: HashMap::new(),
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

                    let player = Player::init(login, client_id.clone());
                    self.players.insert(client_id, player);

                    Console::send_message(format!(
                        "Client \"{}\" connected",
                        self.get_player(client_id).get_login()
                    ));
                }
                ServerEvent::ClientDisconnected(client_id) => {
                    Console::send_message(format!(
                        "Client \"{}\" disconnected",
                        self.get_player(client_id).get_login()
                    ));
                }
            }
        }

        for console_output in NETWORK_CONSOLE_OUTPUT.1.try_iter() {
            self.server.send_message(
                console_output.client_id,
                DefaultChannel::Reliable,
                console_output.message,
            );
        }

        for client_id in self.server.clients_id().into_iter() {
            while let Some(message) = self.server.receive_message(client_id, DefaultChannel::Reliable) {
                let data: ClentMessages = match bincode::options().deserialize(&message) {
                    Ok(d) => d,
                    Err(e) => {
                        Console::send_message(format!("Can't read a message: {:?}", e));
                        continue;
                    }
                };
                match data {
                    ClentMessages::ConsoleCommand { command } => {
                        CONSOLE_HANDLER
                            .lock()
                            .unwrap()
                            .execute_command(self.get_player(client_id), command);
                    }
                }
            }
        }

        self.server.send_packets().unwrap();
        thread::sleep(Duration::from_millis(50));
    }

    pub fn send_console_message(client_id: u64, message: Vec<u8>) {
        NETWORK_CONSOLE_OUTPUT
            .0
            .send(ConsoleOutput::init(client_id, message))
            .unwrap();
    }

    pub fn stop(&mut self) {
        Console::send_message("Stopping the server\n".to_string());
    }
}
