use bevy::prelude::Resource;
use bevy::prelude::*;
use bevy_app::App;
use bevy_ecs::system::Res;
use common::{
    chunks::utils::unpack_network_sectioins,
    network::{channels::ServerChannel, connection_config, login::Login, messages::ServerMessages, PROTOCOL_ID},
};
use parking_lot::RwLock;
use renet::{
    transport::{ClientAuthentication, NetcodeClientTransport},
    RenetClient,
};
use std::net::UdpSocket;
use std::{sync::Arc, time::SystemTime};

use super::events::{
    on_chunk_loaded::{on_chunk_loaded, ChunkLoadedEvent},
    on_chunk_unloaded::{on_chunk_unloaded, ChunkUnloadedEvent},
    on_player_teleport::{on_player_teleport, PlayerTeleportEvent},
    on_resource_loaded::{on_resource_loaded, ResourceLoadedEvent},
};

#[derive(Default)]
pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_events_system);

        app.add_event::<ResourceLoadedEvent>();
        app.add_systems(Update, on_resource_loaded.after(handle_events_system));

        app.add_event::<PlayerTeleportEvent>();
        app.add_systems(Update, on_player_teleport.after(handle_events_system));

        app.add_event::<ChunkLoadedEvent>();
        app.add_systems(Update, on_chunk_loaded.after(handle_events_system));

        app.add_event::<ChunkUnloadedEvent>();
        app.add_systems(Update, on_chunk_unloaded.after(handle_events_system));

        NetworkPlugin::connect(app, "127.0.0.1:14191".to_string(), "Test_cl".to_string()).unwrap();
    }
}

pub type ServerLock = Arc<RwLock<RenetClient>>;
pub type TransferLock = Arc<RwLock<NetcodeClientTransport>>;

#[derive(Resource)]
pub struct NetworkContainer {
    pub client: ServerLock,
    pub transport: TransferLock,
}

impl NetworkPlugin {
    pub fn connect(app: &mut App, ip_port: String, login: String) -> Result<(), String> {
        let network_container = match NetworkPlugin::create_client(ip_port, login) {
            Ok(n) => n,
            Err(e) => return Err(e),
        };
        app.insert_resource(network_container);
        Ok(())
    }

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

        Ok(NetworkContainer {
            client: Arc::new(RwLock::new(client)),
            transport: Arc::new(RwLock::new(transport)),
        })
    }
}

fn handle_events_system(
    network_container: Res<NetworkContainer>,
    time: Res<Time>,
    mut resource_loaded_event: EventWriter<ResourceLoadedEvent>,
    mut player_teleport_event: EventWriter<PlayerTeleportEvent>,
    mut chunk_loaded_event: EventWriter<ChunkLoadedEvent>,
    mut chunk_unloaded_event: EventWriter<ChunkUnloadedEvent>,
) {
    let mut client = network_container.client.as_ref().write();
    let mut transport = network_container.transport.as_ref().write();

    client.update(time.delta());
    transport.update(time.delta(), &mut client).unwrap();

    if !client.is_disconnected() {
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
    }

    transport.send_packets(&mut client).unwrap();
}
