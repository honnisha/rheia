use bevy::prelude::*;
use bevy_app::App;
use bevy_ecs::system::Res;
use common::chunks::chunk_position::ChunkPosition;
use network::client::IClientNetwork;
use network::messages::{ClientMessages, NetworkMessageType, ServerMessages};
use network::NetworkClient;
use parking_lot::RwLock;

use std::sync::Arc;

use crate::client_scripts::resource_manager::ResourceManager;
use crate::utils::bridge::IntoBevyVector;
use crate::world::worlds_manager::WorldsManager;
use crate::VERSION;

use super::events::{
    self, netcode_error::NetcodeErrorEvent, on_chunk_loaded::ChunkLoadedEvent, on_chunk_unloaded::ChunkUnloadedEvent,
    on_player_teleport::PlayerTeleportEvent,
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

        let io_loop = tokio::runtime::Runtime::new().unwrap();
        let result = io_loop.block_on(async { NetworkClient::new(ip_port).await });

        let network = match result {
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
    mut resource_manager: NonSendMut<ResourceManager>,
    mut worlds_manager: ResMut<WorldsManager>,
    mut netcode_error_event: EventWriter<NetcodeErrorEvent>,
    mut player_teleport_event: EventWriter<PlayerTeleportEvent>,
    mut chunk_loaded_event: EventWriter<ChunkLoadedEvent>,
    mut chunk_unloaded_event: EventWriter<ChunkUnloadedEvent>,
) {
    let lock = network_container.get_network_lock();
    let network = lock.read();

    // Recieve errors from network thread
    for error in network.iter_errors() {
        netcode_error_event.write(NetcodeErrorEvent::new(error));
        return;
    }

    let mut chunks: Vec<ChunkPosition> = Default::default();
    for decoded in network.iter_server_messages() {
        match decoded {
            ServerMessages::AllowConnection => {
                let connection_info = ClientMessages::ConnectionInfo {
                    login: "Test_cl".to_string(),
                    version: VERSION.to_string(),
                    architecture: "-".to_string(),
                    rendering_device: "-".to_string(),
                };
                network.send_message(NetworkMessageType::ReliableOrdered, &connection_info);
            }
            ServerMessages::ConsoleOutput { message } => {
                log::info!(target: "network", "{}", message);
            }
            ServerMessages::ResourcesScheme { list, archive_hash } => {
                resource_manager.set_resource_scheme(list, archive_hash);
                let (scripts_count, media_count) = resource_manager.get_resource_scheme_count();
                log::info!(target: "network", "Resources scheme loaded from network (scripts:{}, media:{})", scripts_count, media_count);
            }
            ServerMessages::ResourcesPart {
                index,
                total,
                data,
                last,
            } => {
                {
                    resource_manager.load_archive_chunk(&mut data);

                    if last {
                        if let Err(e) = resource_manager.load_archive() {
                            let msg = format!("Network resources download error: {}", e);
                            netcode_error_event.write(NetcodeErrorEvent::new(msg));
                            return;
                        }
                        log::info!(target: "network", "Resources archive downloading from the network; index:{}", index);
                    }
                }

                let msg = ClientMessages::ResourcesLoaded { last_index: index };
                network.send_message(NetworkMessageType::ReliableOrdered, &msg);
            }
            ServerMessages::Settings { block_types } => {
                log::info!(target: "network", "Recieved settings from the network");

                {
                    let mut block_storage = worlds_manager.get_block_storage_mut();
                    if let Err(e) =
                        block_storage.load_blocks_types(block_types, &*resource_manager.get_resources_storage())
                    {
                        return Err(e);
                    }
                }

                if let Err(e) = worlds_manager.build_textures(&*&resource_manager.get_resources_storage()) {
                    return Err(e);
                }

                network.send_message(NetworkMessageType::ReliableOrdered, &ClientMessages::SettingsLoaded);

                // main.on_server_connected();
            }

            ServerMessages::SpawnWorld { world_slug } => todo!(),
            ServerMessages::UpdatePlayerSkin { skin } => todo!(),
            ServerMessages::Teleport {
                world_slug,
                position,
                rotation,
            } => {
                player_teleport_event.send(PlayerTeleportEvent::new(world_slug, position.to_transform(), rotation));
            }

            ServerMessages::ChunkSectionInfo {
                world_slug,
                chunk_position,
                sections,
            } => todo!(),
            ServerMessages::UnloadChunks { world_slug, chunks } => todo!(),

            ServerMessages::StartStreamingEntity {
                world_slug,
                id,
                position,
                rotation,
                skin,
            } => todo!(),
            ServerMessages::UpdateEntitySkin { world_slug, id, skin } => todo!(),
            ServerMessages::StopStreamingEntities { world_slug, ids } => todo!(),
            ServerMessages::EntityMove {
                world_slug,
                id,
                position,
                rotation,
            } => todo!(),

            ServerMessages::EditBlock {
                world_slug,
                position,
                new_block_info,
            } => todo!(),
            /*
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
            */
        }
    }
    if chunks.len() > 0 {
        let input = ClientMessages::ChunkRecieved {
            chunk_positions: chunks,
        };
        network.send_message(NetworkMessageType::WorldInfo, &input);
    }
}
