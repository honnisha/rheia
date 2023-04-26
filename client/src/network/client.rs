use bincode::Options;
use common::network_messages::{ClentMessages, ClientLogin, ServerMessages};
use renet::{ClientAuthentication, RenetClient, RenetConnectionConfig, DefaultChannel};
use std::time::Duration;
use std::{net::UdpSocket, time::SystemTime};

use crate::client_scripts::resource_manager::ResourceManager;
use crate::console::console_handler::Console;

pub const PROTOCOL_ID: u64 = 7;
pub const CHANNEL_ID: u8 = 0;

fn get_network_client(ip_port: String, login: ClientLogin) -> RenetClient {
    let server_addr = ip_port.clone().parse().unwrap();

    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();

    let connection_config = RenetConnectionConfig::default();

    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

    let client_id = current_time.as_millis() as u64;

    let authentication = ClientAuthentication::Unsecure {
        protocol_id: PROTOCOL_ID,
        client_id,
        server_addr,
        user_data: Some(login.to_netcode_user_data()),
    };
    let client = RenetClient::new(current_time, socket, connection_config, authentication).unwrap();
    client
}

pub struct NetworkClient {
    client: Option<RenetClient>,
}

impl NetworkClient {
    pub fn init() -> Self {
        NetworkClient {
            client: None,
        }
    }

    pub fn create_client(&mut self, ip_port: String, login: String) {
        Console::send_message(format!("Start network client for {}", ip_port));
        self.client = Some(get_network_client(ip_port, ClientLogin(login)));
    }

    fn get_client(&mut self) -> &mut RenetClient {
        match self.client.as_mut() {
            Some(c) => c,
            None => panic!("client is not initialized"),
        }
    }

    pub fn update(&mut self, delta: f64, resource_manager: &mut ResourceManager) {
        if let Err(e) = self.get_client().update(Duration::from_secs_f64(delta)) {
            panic!("Connection error: {}", e);
        }

        if self.get_client().is_connected() {
            while let Some(message) = self.get_client().receive_message(CHANNEL_ID) {
                let data: ServerMessages = match bincode::options().deserialize(&message) {
                    Ok(d) => d,
                    Err(e) => {
                        Console::send_message(format!("Can't read a message: {:?}", e));
                        continue;
                    }
                };
                match data {
                    ServerMessages::ConsoleOutput { text } => Console::send_message(text),
                    ServerMessages::ResourceCallbackTrigger { callback_name, args } => {
                        resource_manager.run_event(&callback_name, &args);
                    },
                    ServerMessages::LoadResource { slug, scripts } => {
                        match resource_manager.try_load(&slug, scripts) {
                            Ok(()) => {
                                Console::send_message(format!("Loaded resource \"{}\"", slug));
                            },
                            Err(e) => {
                                Console::send_message(format!("Resource error: {}", e));
                                self.send_resource_load_error(e);
                            },
                        }
                    },

                }
            }
        }
        self.get_client().send_packets().unwrap();
    }

    pub fn disconnect(&mut self) {
        self.get_client().disconnect();
        Console::send_message("Disconnected from the server".to_string());
    }

    pub fn send_resource_load_error(&mut self, error: String) {
        match bincode::options().serialize(&ClentMessages::LoadResourceError { text: error }) {
            Ok(message) => self.get_client().send_message(DefaultChannel::Reliable, message),
            Err(e) => {
                Console::send_message(format!("Error serialize resource load error: {}", e));
            }
        }
    }

    pub fn send_console_command(&mut self, command: String) {
        match bincode::options().serialize(&ClentMessages::ConsoleCommand { command }) {
            Ok(message) => self.get_client().send_message(DefaultChannel::Reliable, message),
            Err(e) => {
                Console::send_message(format!("Error console command send: {}", e));
            }
        }
    }
}
