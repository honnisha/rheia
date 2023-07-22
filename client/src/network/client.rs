use crate::main_scene::Main;
use common::network::connection_config;
use common::network::ClientChannel;
use common::network::ClientMessages;
use common::network::Login;
use common::network::ServerChannel;
use common::network::ServerMessages;
use common::network::PROTOCOL_ID;
use lazy_static::lazy_static;
use log::error;
use log::info;
use renet::transport::ClientAuthentication;
use renet::transport::NetcodeClientTransport;
use renet::RenetClient;
use std::net::UdpSocket;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;
use std::time::SystemTime;

lazy_static! {
    static ref NETWORK_CONTAINER: Arc<RwLock<NetworkContainer>> = Arc::new(RwLock::new(NetworkContainer::default()));
}

pub struct NetworkContainer {
    pub client: Option<Arc<RwLock<RenetClient>>>,
    pub transport: Option<Arc<RwLock<NetcodeClientTransport>>>,
}

impl Default for NetworkContainer {
    fn default() -> Self {
        NetworkContainer {
            client: None,
            transport: None,
        }
    }
}

impl NetworkContainer {
    pub fn create_client(ip_port: String, login: String) -> Result<(), String> {
        info!("Connecting to the server at {}", ip_port);
        let mut network_handler = NETWORK_CONTAINER.write().unwrap();

        let client = RenetClient::new(connection_config());
        network_handler.client = Some(Arc::new(RwLock::new(client)));

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

        network_handler.transport = Some(Arc::new(RwLock::new(
            NetcodeClientTransport::new(current_time, authentication, socket).unwrap(),
        )));

        Ok(())
    }

    pub fn update(delta: f64, main_scene: &mut Main) -> Result<(), String> {
        let delta_time = Duration::from_secs_f64(delta);
        let container = NETWORK_CONTAINER.read().unwrap();

        let mut client = container.client.as_ref().unwrap().write().unwrap();
        let mut transport = container.transport.as_ref().unwrap().write().unwrap();

        client.update(delta_time);
        transport.update(delta_time, &mut client).unwrap();

        if !client.is_disconnected() {
            while let Some(server_message) = client.receive_message(ServerChannel::Messages) {
                let decoded: ServerMessages = match bincode::deserialize(&server_message) {
                    Ok(d) => d,
                    Err(e) => {
                        error!("Decode server message error: {}", e);
                        continue;
                    }
                };
                match decoded {
                    ServerMessages::ConsoleOutput { message } => {
                        info!("{}", message);
                    },
                    ServerMessages::Resource { slug, scripts } => {
                        let resource_manager = main_scene.get_resource_manager_mut();
                        info!("Start loading client resource slug:\"{}\"", slug);
                        match resource_manager.try_load(&slug, scripts) {
                            Ok(_) => {
                                info!("Client resource slug:\"{}\" loaded", slug);
                            },
                            Err(e) => {
                                error!("Client resource slug:\"{}\" error: {}", slug, e);
                            },
                        }
                    },
                    ServerMessages::Teleport { world_slug, location } => {
                        main_scene.teleport_player(world_slug, location);
                    },
                    ServerMessages::ChunkSectionInfo { chunk_position, sections } => {
                        main_scene.world_manager.load_chunk(chunk_position, sections);
                        info!("chunk revieved: {:?}", chunk_position);
                    },
                }
            }
        }

        match transport.send_packets(&mut client) {
            Ok(_) => Ok(()),
            Err(e) => {
                Err(e.to_string())
            }
        }
    }

    pub fn disconnect() {
        let container = NETWORK_CONTAINER.read().unwrap();

        let mut transport = container.transport.as_ref().unwrap().write().unwrap();
        if transport.is_connected() {
            transport.disconnect();
            info!("{}", "Disconnected from the server");
        }
    }

    pub fn send_console_command(command: String) {
        let container = NETWORK_CONTAINER.read().unwrap();

        let mut client = container.client.as_ref().unwrap().write().unwrap();
        let input = ClientMessages::ConsoleInput { command: command };
        let command_message = bincode::serialize(&input).unwrap();
        client.send_message(ClientChannel::Messages, command_message);
    }
}
