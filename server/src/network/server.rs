use bevy::time::Time;
use bevy_app::AppExit;
use bevy_ecs::{system::{Resource, ResMut, Res, Commands}, prelude::EventWriter, world::World};
use bincode::Options;
use common::network_messages::{ClentMessages, ClientLogin, ServerMessages};
use lazy_static::lazy_static;
use renet::{DefaultChannel, RenetConnectionConfig, RenetServer, ServerAuthentication, ServerConfig, ServerEvent};
use std::{
    collections::HashMap,
    net::UdpSocket,
    thread,
    time::{Duration, SystemTime}, sync::{Arc, atomic::{AtomicBool, Ordering}},
};

use super::player::PlayerNetwork;
use crate::{client_resources::resources_manager::ResourceManager, console_send, console::console_handler::ConsoleHandler};
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

#[derive(Resource)]
pub struct NetworkServer {
    server: RenetServer,
    players: HashMap<u64, PlayerNetwork>,
}

impl NetworkServer {
    pub fn get_server(&mut self) -> &mut RenetServer {
        &mut self.server
    }
}

#[derive(Resource)]
pub struct ServerRuntime {
    pub server_active: Arc<AtomicBool>,
}

impl ServerRuntime {
    pub fn new() -> Self {
        Self {
            server_active: Arc::new(AtomicBool::new(true)),
        }
    }
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
    fn get_player(&self, client_id: u64) -> &PlayerNetwork {
        &self.players[&client_id]
    }

    pub fn init(ip_port: String) -> Self {
        console_send(format!("Start network server for {}", ip_port));
        NetworkServer {
            server: get_network_server(ip_port),
            players: HashMap::new(),
        }
    }

    pub fn update_tick(
        mut commands: Commands,
        mut network_server: ResMut<NetworkServer>,
        server_runtime: Res<ServerRuntime>,
        resource_manager: &ResourceManager,
        time: Res<Time>,
    ) {
        if !server_runtime.server_active.load(Ordering::Relaxed) {
            return;
        }

        let mut server = network_server.get_server();

        // Receive new messages and update clients
        server.update(time.delta()).unwrap();

        // Check for client connections/disconnections
        while let Some(event) = server.get_event() {
            match event {
                ServerEvent::ClientConnected(client_id, user_data) => {
                    let login = ClientLogin::from_user_data(&user_data).0;
                    commands.spawn((
                        PlayerNetwork::init(login.clone(), client_id.clone()),
                    ));
                    NetworkServer::send_resources(server, client_id, resource_manager);

                    console_send(format!("Client \"{}\" connected", login));
                }
                ServerEvent::ClientDisconnected(client_id) => {
                    console_send(format!(
                        "Client \"{}\" disconnected",
                        self.get_player(client_id).get_login()
                    ));
                }
            }
        }

        for console_output in NETWORK_CONSOLE_OUTPUT.1.try_iter() {
            server.send_message(
                console_output.client_id,
                DefaultChannel::Reliable,
                console_output.message,
            );
        }

        for client_id in server.clients_id().into_iter() {
            while let Some(message) = server.receive_message(client_id, DefaultChannel::Reliable) {
                let data: ClentMessages = match bincode::options().deserialize(&message) {
                    Ok(d) => d,
                    Err(e) => {
                        console_send(format!("Can't read a message: {:?}", e));
                        continue;
                    }
                };
                match data {
                    ClentMessages::ConsoleCommand { command } => {
                        ConsoleHandler::execute_command(self.get_player(client_id), command);
                    }
                    ClentMessages::LoadResourceError { text } => {
                        console_send(format!(
                            "User \"{}\" get resource error: {}",
                            self.get_player(client_id).get_login(),
                            text
                        ));
                    }
                }
            }
        }

        server.send_packets().unwrap();
        //thread::sleep(Duration::from_millis(50));
    }

    fn send_resources(server: &mut RenetServer, client_id: u64, resource_manager: &ResourceManager) {
        for (slug, resource_instance) in resource_manager.get_resources().iter() {
            let data = ServerMessages::LoadResource {
                slug: slug.clone(),
                scripts: resource_instance.get_client_scripts().clone(),
            };
            let message = match bincode::options().serialize(&data) {
                Ok(m) => m,
                Err(_) => {
                    console_send(format!("Error serialize resource {}", slug));
                    continue;
                }
            };
            server.send_message(client_id, DefaultChannel::Reliable, message);
        }
    }

    pub fn send_console_message(client_id: u64, message: Vec<u8>) {
        NETWORK_CONSOLE_OUTPUT
            .0
            .send(ConsoleOutput::init(client_id, message))
            .unwrap();
    }

    pub fn stop(
        mut network_server: ResMut<NetworkServer>,
        server_runtime: Res<ServerRuntime>,
        mut exit: EventWriter<AppExit>,
    ) {
        console_send("Stopping the server\n".to_string());
        thread::sleep(Duration::from_millis(50));
        exit.send(AppExit);
    }
}
