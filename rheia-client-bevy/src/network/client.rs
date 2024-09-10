use bevy::prelude::Resource;
use bevy::prelude::*;
use bevy_app::App;
use bevy_ecs::system::Res;
use common::{
    chunks::chunk_position::ChunkPosition,
    network::{
        client::ClientNetwork,
        messages::{ClientMessages, NetworkMessageType, ServerMessages}, NetworkClient,
    },
};
use parking_lot::RwLock;

use std::sync::Arc;

use crate::{utils::bridge::IntoBevyVector};

use super::events::{
    self, netcode_error::NetcodeErrorEvent, on_chunk_loaded::ChunkLoadedEvent, on_chunk_unloaded::ChunkUnloadedEvent,
    on_player_teleport::PlayerTeleportEvent, on_resource_loaded::ResourceLoadedEvent,
};

#[derive(Default)]
pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NetcodeErrorEvent>();
        app.add_systems(
            Update,
            events::netcode_error::netcode_error_handler.after(handle_events_system),
        );

        app.add_event::<ResourceLoadedEvent>();
        app.add_systems(
            Update,
            events::on_resource_loaded::on_resource_loaded.after(handle_events_system),
        );

        app.add_event::<PlayerTeleportEvent>();
        app.add_systems(
            Update,
            events::on_player_teleport::on_player_teleport.after(handle_events_system),
        );

        app.add_event::<ChunkLoadedEvent>();
        app.add_systems(
            Update,
            events::on_chunk_loaded::on_chunk_loaded.after(events::on_player_teleport::on_player_teleport),
        );

        app.add_event::<ChunkUnloadedEvent>();
        app.add_systems(
            Update,
            events::on_chunk_unloaded::on_chunk_unloaded.after(events::on_player_teleport::on_player_teleport),
        );

        app.add_systems(Update, handle_events_system.run_if(resource_exists::<NetworkContainer>));
        app.add_systems(Startup, connect_server);
    }
}

pub type NetworkLockType = Arc<RwLock<NetworkClient>>;

#[derive(Resource)]
pub struct NetworkContainer {
    client_network: Arc<RwLock<NetworkClient>>,
}

impl NetworkContainer {
    pub fn new(ip_port: String) -> Result<Self, String> {
        log::info!(target: "network", "Connecting to the server at {}", ip_port);
        let network = match NetworkClient::new(ip_port) {
            Ok(n) => n,
            Err(e) => return Err(e),
        };
        Ok(Self {
            client_network: Arc::new(RwLock::new(network)),
        })
    }

    pub fn get_network_lock(&self) -> Arc<RwLock<NetworkClient>> {
        self.client_network.clone()
    }

    pub fn disconnect(&self) {
        let network = self.client_network.read();

        if network.is_connected() {
            log::info!(target: "network", "Disconnected from the server");
            network.disconnect();
        }
    }
}

fn connect_server(mut commands: Commands) {
    let ip_port = "127.0.0.1:19132".to_string();

    let network_container = match NetworkContainer::new(ip_port) {
        Ok(c) => c,
        Err(e) => {
            log::error!(target: "network", "Network connection error: {}", e);
            panic!("Connection error: {}", e);
        }
    };

    commands.insert_resource(network_container);
    log::info!(target: "network", "Connection to the server was successful");
}

fn handle_events_system(
    network_container: Res<NetworkContainer>,
    mut netcode_error_event: EventWriter<NetcodeErrorEvent>,
    mut resource_loaded_event: EventWriter<ResourceLoadedEvent>,
    mut player_teleport_event: EventWriter<PlayerTeleportEvent>,
    mut chunk_loaded_event: EventWriter<ChunkLoadedEvent>,
    mut chunk_unloaded_event: EventWriter<ChunkUnloadedEvent>,
) {
    let lock = network_container.get_network_lock();
    let network = lock.read();

    // Recieve errors from network thread
    for error in network.iter_errors() {
        netcode_error_event.send(NetcodeErrorEvent::new(error));
        return;
    }

    let mut chunks: Vec<ChunkPosition> = Default::default();
    for decoded in network.iter_server_messages() {
        match decoded {
            ServerMessages::AllowConnection => {
                let connection_info = ClientMessages::ConnectionInfo {
                    login: "Test_cl".to_string(),
                };
                network.send_message(&connection_info, NetworkMessageType::ReliableOrdered);
            }
            ServerMessages::ConsoleOutput { message } => {
                log::info!(target: "network", "{}", message);
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
                    location.to_transform(),
                    yaw,
                    pitch,
                ));
            }
            ServerMessages::ChunkSectionInfo {
                world_slug,
                chunk_position,
                sections,
            } => {
                chunk_loaded_event.send(ChunkLoadedEvent::new(world_slug, chunk_position, sections));
                chunks.push(chunk_position);
            }
            ServerMessages::UnloadChunks { world_slug, chunks } => {
                chunk_unloaded_event.send(ChunkUnloadedEvent::new(world_slug, chunks));
            }
            _ => panic!("unsupported message"),
        }
    }
    if chunks.len() > 0 {
        let input = ClientMessages::ChunkRecieved {
            chunk_positions: chunks,
        };
        network.send_message(&input, NetworkMessageType::WorldInfo);
    }
}
