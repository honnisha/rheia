use bevy::prelude::Resource;
use bevy::prelude::*;
use bevy_app::App;
use bevy_ecs::system::Res;
use common::{
    chunks::utils::unpack_network_sectioins,
    network::{channels::ServerChannel, connection_config, login::Login, messages::ServerMessages, PROTOCOL_ID},
};
use log::info;
use renet::{
    transport::{ClientAuthentication, NetcodeClientTransport},
    RenetClient,
};
use std::{
    net::UdpSocket,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};
use std::{sync::Arc, time::SystemTime};

use super::events::{
    netcode_error::{netcode_error_handler, NetcodeErrorEvent},
    on_chunk_loaded::{on_chunk_loaded, ChunkLoadedEvent},
    on_chunk_unloaded::{on_chunk_unloaded, ChunkUnloadedEvent},
    on_player_teleport::{on_player_teleport, PlayerTeleportEvent},
    on_resource_loaded::{on_resource_loaded, ResourceLoadedEvent},
};

#[derive(Default)]
pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_events_system.run_if(resource_exists::<NetworkContainer>()),
        );

        app.add_event::<NetcodeErrorEvent>();
        app.add_systems(Update, netcode_error_handler.after(handle_events_system));

        app.add_event::<ResourceLoadedEvent>();
        app.add_systems(Update, on_resource_loaded.after(handle_events_system));

        app.add_event::<PlayerTeleportEvent>();
        app.add_systems(Update, on_player_teleport.after(handle_events_system));

        app.add_event::<ChunkLoadedEvent>();
        app.add_systems(Update, on_chunk_loaded.after(on_player_teleport));

        app.add_event::<ChunkUnloadedEvent>();
        app.add_systems(Update, on_chunk_unloaded.after(on_player_teleport));

        app.add_systems(Startup, connect_server);
    }
}

pub type ServerLock = Arc<RwLock<RenetClient>>;
pub type TransferLock = Arc<RwLock<NetcodeClientTransport>>;

#[derive(Resource)]
pub struct NetworkContainer {
    client: ServerLock,
    transport: TransferLock,
}

impl NetworkContainer {
    pub fn new(client: RenetClient, transport: NetcodeClientTransport) -> Self {
        Self {
            client: Arc::new(RwLock::new(client)),
            transport: Arc::new(RwLock::new(transport)),
        }
    }

    pub fn disconnect(&self) {
        let mut transport = self.get_transport_mut();
        if transport.is_connected() {
            transport.disconnect();
            info!("Disconnected from the server");
        }
    }

    pub fn get_client_mut(&self) -> RwLockWriteGuard<RenetClient> {
        self.client.as_ref().write().expect("poisoned")
    }

    pub fn get_transport(&self) -> RwLockReadGuard<NetcodeClientTransport> {
        self.transport.as_ref().read().expect("poisoned")
    }

    pub fn get_transport_mut(&self) -> RwLockWriteGuard<NetcodeClientTransport> {
        self.transport.as_ref().write().expect("poisoned")
    }
}

impl NetworkPlugin {
    fn create_client(ip_port: String, login: String) -> Result<NetworkContainer, String> {
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

        Ok(NetworkContainer::new(client, transport))
    }
}

fn connect_server(mut commands: Commands) {
    let ip_port = "127.0.0.1:14191".to_string();
    let login = "Test_cl".to_string();
    info!("Connecting to the server {}", ip_port);
    let network_container = match NetworkPlugin::create_client(ip_port, login) {
        Ok(n) => n,
        Err(e) => {
            panic!("Connection error: {}", e);
        }
    };
    commands.insert_resource(network_container);
    info!("Connection to the server was successful");
}

fn handle_events_system(
    network_container: Res<NetworkContainer>,
    time: Res<Time>,
    mut netcode_error_event: EventWriter<NetcodeErrorEvent>,
    mut resource_loaded_event: EventWriter<ResourceLoadedEvent>,
    mut player_teleport_event: EventWriter<PlayerTeleportEvent>,
    mut chunk_loaded_event: EventWriter<ChunkLoadedEvent>,
    mut chunk_unloaded_event: EventWriter<ChunkUnloadedEvent>,
) {
    let delta = time.delta();

    let mut client = network_container.get_client_mut();
    if client.is_disconnected() {
        return;
    }
    client.update(delta);

    let mut transport = network_container.get_transport_mut();
    if let Err(e) = transport.update(delta, &mut client) {
        netcode_error_event.send(NetcodeErrorEvent::new(e));
        return;
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
                resource_loaded_event.send(ResourceLoadedEvent::new(slug, scripts));
            }
            ServerMessages::Teleport {
                world_slug,
                location,
                yaw,
                pitch,
            } => {
                player_teleport_event.send(PlayerTeleportEvent::new(
                    world_slug,
                    Transform::from_xyz(location[0], location[1], location[2]),
                    yaw,
                    pitch,
                ));
            }
            ServerMessages::ChunkSectionInfo {
                chunk_position,
                mut sections,
            } => {
                chunk_loaded_event.send(ChunkLoadedEvent::new(
                    chunk_position,
                    unpack_network_sectioins(&mut sections),
                ));
            }
            ServerMessages::UnloadChunks { chunks } => {
                chunk_unloaded_event.send(ChunkUnloadedEvent::new(chunks));
            }
        }
    }

    if let Err(e) = transport.send_packets(&mut client) {
        netcode_error_event.send(NetcodeErrorEvent::new(e));
        return;
    }
}
