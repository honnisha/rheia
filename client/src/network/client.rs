use crate::controller::player_controller::PlayerMovement;
use crate::entities::position::GodotPositionConverter;
use crate::main_scene::Main;
use common::chunks::utils::unpack_network_sectioins;
use common::network::channels::ClientChannel;
use common::network::channels::ServerChannel;
use common::network::connection_config;
use common::network::login::Login;
use common::network::messages::ClientMessages;
use common::network::messages::ServerMessages;
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
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;
use std::time::Duration;
use std::time::SystemTime;

lazy_static! {
    static ref NETWORK_CONTAINER: Arc<RwLock<NetworkContainer>> = Arc::new(RwLock::new(NetworkContainer::default()));
}

#[derive(Default)]
pub struct NetworkContainer {
    client: Option<Arc<RwLock<RenetClient>>>,
    transport: Option<Arc<RwLock<NetcodeClientTransport>>>,
}

impl NetworkContainer {
    pub fn new(client: RenetClient, transport: NetcodeClientTransport) -> Self {
        Self {
            client: Some(Arc::new(RwLock::new(client))),
            transport: Some(Arc::new(RwLock::new(transport))),
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
        *network_handler = NetworkContainer::new(client, transport);

        Ok(())
    }

    pub fn read() -> RwLockReadGuard<'static, NetworkContainer> {
        NETWORK_CONTAINER.read().unwrap()
    }

    pub fn get_client_mut(&self) -> RwLockWriteGuard<RenetClient> {
        self.client.as_ref().unwrap().write().expect("poisoned")
    }

    pub fn _get_transport(&self) -> RwLockReadGuard<NetcodeClientTransport> {
        self.transport.as_ref().unwrap().read().expect("poisoned")
    }

    pub fn get_transport_mut(&self) -> RwLockWriteGuard<NetcodeClientTransport> {
        self.transport.as_ref().unwrap().write().expect("poisoned")
    }

    pub fn update(delta: f64, main_scene: &mut Main) -> Result<(), String> {
        let delta_time = Duration::from_secs_f64(delta);
        let container = NetworkContainer::read();

        let mut client = container.get_client_mut();
        if client.is_disconnected() {
            return Err("disconnected".to_string());
        }
        let mut transport = container.get_transport_mut();

        client.update(delta_time);
        if let Err(e) = transport.update(delta_time, &mut client) {
            return Err(e.to_string());
        }

        while let Some(server_message) = client.receive_message(ServerChannel::Reliable) {
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
                }
                ServerMessages::Resource { slug, scripts } => {
                    let resource_manager = main_scene.get_resource_manager_mut();
                    info!("Start loading client resource slug:\"{}\"", slug);
                    match resource_manager.try_load(&slug, scripts) {
                        Ok(_) => {
                            info!("Client resource slug:\"{}\" loaded", slug);
                        }
                        Err(e) => {
                            error!("Client resource slug:\"{}\" error: {}", slug, e);
                        }
                    }
                }
                ServerMessages::Teleport {
                    world_slug,
                    location,
                    yaw,
                    pitch,
                } => {
                    main_scene.teleport_player(
                        world_slug,
                        GodotPositionConverter::vec3_from_array(&location),
                        yaw,
                        pitch,
                    );
                }
                ServerMessages::ChunkSectionInfo {
                    chunk_position,
                    mut sections,
                } => {
                    main_scene
                        .get_world_manager_mut()
                        .load_chunk(chunk_position, unpack_network_sectioins(&mut sections));
                }
                ServerMessages::UnloadChunks { chunks } => {
                    main_scene.get_world_manager_mut().unload_chunk(chunks);
                }
            }
        }

        if let Err(e) = transport.send_packets(&mut client) {
            return Err(e.to_string());
        }
        return Ok(());
    }

    pub fn disconnect() {
        let container = NetworkContainer::read();

        let mut transport = container.get_transport_mut();
        if transport.is_connected() {
            transport.disconnect();
            info!("{}", "Disconnected from the server");
        }
    }

    pub fn send_console_command(command: String) {
        let container = NetworkContainer::read();

        let mut client = container.get_client_mut();
        let input = ClientMessages::ConsoleInput { command: command };
        let command_message = bincode::serialize(&input).unwrap();
        client.send_message(ClientChannel::Reliable, command_message);
    }

    pub fn send_player_move(movement: PlayerMovement) {
        let container = NetworkContainer::read();

        let mut client = container.get_client_mut();
        let message = bincode::serialize(&movement.into_network()).unwrap();
        client.send_message(ClientChannel::Unreliable, message);
    }
}
