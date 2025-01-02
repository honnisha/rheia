use std::thread;

use bevy::prelude::IntoSystemConfigs;
use bevy::time::Time;
use bevy_app::{App, Update};
use bevy_ecs::change_detection::Mut;
use bevy_ecs::{
    prelude::EventWriter,
    system::{Res, ResMut, Resource},
    world::World,
};
use flume::{Receiver, Sender};
use lazy_static::lazy_static;
use network::messages::{ClientMessages, NetworkMessageType, ServerMessages};
use network::server::{ConnectionMessages, IServerConnection, IServerNetwork};
use network::NetworkServer;

use super::events::{
    on_connection::{on_connection, PlayerConnectionEvent},
    on_connection_info::{on_connection_info, PlayerConnectionInfoEvent},
    on_disconnect::{on_disconnect, PlayerDisconnectEvent},
    on_edit_block::{on_edit_block, EditBlockEvent},
    on_media_loaded::{on_media_loaded, PlayerMediaLoadedEvent},
    on_player_move::{on_player_move, PlayerMoveEvent},
    on_settings_loaded::{on_settings_loaded, PlayerSettingsLoadedEvent},
};
use crate::entities::entity::{IntoServerPosition, IntoServerRotation};
use crate::entities::events::on_player_spawn::on_player_spawn;
use crate::network::chunks_sender::send_chunks;
use crate::network::client_network::ClientNetwork;
use crate::network::clients_container::ClientsContainer;
use crate::network::sync_players::PlayerSpawnEvent;
use crate::{console::commands_executer::CommandsHandler, LaunchSettings};

const MIN_TICK_TIME: std::time::Duration = std::time::Duration::from_millis(50);

pub struct NetworkPlugin;

lazy_static! {
    static ref CONSOLE_INPUT: (Sender<(u64, String)>, Receiver<(u64, String)>) = flume::unbounded();
}

#[derive(Resource)]
pub struct NetworkContainer {
    server_network: Box<NetworkServer>,
}

impl NetworkContainer {
    pub fn new(ip_port: String) -> Self {
        let io_loop = tokio::runtime::Runtime::new().unwrap();
        let network = io_loop.block_on(async { NetworkServer::new(ip_port).await });
        Self {
            server_network: Box::new(network),
        }
    }

    pub fn is_connected(&self, client: &ClientNetwork) -> bool {
        let network = self.server_network.as_ref();
        network.is_connected(client.get_connection())
    }
}

impl NetworkPlugin {
    pub fn build(app: &mut App) {
        let server_settings = app.world().get_resource::<LaunchSettings>().unwrap();
        let ip_port = format!("{}:{}", server_settings.get_args().ip, server_settings.get_args().port);

        log::info!(target: "network", "Starting server on {}", ip_port);

        app.insert_resource(NetworkContainer::new(ip_port));
        app.insert_resource(ClientsContainer::default());

        app.add_systems(Update, receive_message_system);
        app.add_systems(Update, handle_events_system);
        app.add_systems(Update, send_chunks.after(handle_events_system));

        app.add_systems(Update, console_client_command_event);

        app.add_event::<PlayerConnectionEvent>();
        app.add_systems(Update, on_connection.after(handle_events_system));

        app.add_event::<PlayerConnectionInfoEvent>();
        app.add_systems(Update, on_connection_info.after(handle_events_system));

        app.add_event::<PlayerDisconnectEvent>();
        app.add_systems(Update, on_disconnect.after(handle_events_system));

        app.add_event::<PlayerMoveEvent>();
        app.add_systems(Update, on_player_move.after(handle_events_system));

        app.add_event::<EditBlockEvent>();
        app.add_systems(Update, on_edit_block.after(handle_events_system));

        app.add_event::<PlayerMediaLoadedEvent>();
        app.add_systems(Update, on_media_loaded.after(handle_events_system));

        app.add_event::<PlayerSettingsLoadedEvent>();
        app.add_systems(Update, on_settings_loaded.after(handle_events_system));

        app.add_event::<PlayerSpawnEvent>();
        app.add_systems(Update, on_player_spawn);
    }

    pub(crate) fn send_console_output(client: &ClientNetwork, message: String) {
        let input = ServerMessages::ConsoleOutput { message: message };
        client.send_message(NetworkMessageType::ReliableOrdered, &input);
    }
}

fn receive_message_system(
    network_container: Res<NetworkContainer>,
    time: Res<Time>,
    clients: Res<ClientsContainer>,
    mut connection_info_events: EventWriter<PlayerConnectionInfoEvent>,
    mut player_move_events: EventWriter<PlayerMoveEvent>,
    mut edit_block_events: EventWriter<EditBlockEvent>,
    mut player_media_loaded_events: EventWriter<PlayerMediaLoadedEvent>,
    mut settings_loaded_events: EventWriter<PlayerSettingsLoadedEvent>,
) {
    #[cfg(feature = "trace")]
    let _span = bevy_utils::tracing::info_span!("receive_message_system").entered();

    // Sleep if the tick rate is less than the minimum tick rate
    if time.delta() < MIN_TICK_TIME {
        thread::sleep(MIN_TICK_TIME - time.delta());
    }

    let network = network_container.server_network.as_ref();

    let io_loop = tokio::runtime::Runtime::new().unwrap();
    io_loop.block_on(async { network.step(time.delta()).await });

    for message in network.drain_errors() {
        log::error!(target: "network", "Network error: {}", message);
    }

    for (client_id, client) in clients.iter() {
        for decoded in client.get_connection().drain_client_messages() {
            match decoded {
                ClientMessages::ResourcesLoaded { last_index } => {
                    let msg = PlayerMediaLoadedEvent::new(client.clone(), Some(last_index));
                    player_media_loaded_events.send(msg);
                }
                ClientMessages::SettingsLoaded => {
                    let msg = PlayerSettingsLoadedEvent::new(client.clone());
                    settings_loaded_events.send(msg);
                }
                ClientMessages::ConsoleInput { command } => {
                    CONSOLE_INPUT.0.send((*client_id, command)).unwrap();
                }
                ClientMessages::ChunkRecieved { chunk_positions } => {
                    client.mark_chunks_as_recieved(chunk_positions);
                }
                ClientMessages::PlayerMove { position, rotation } => {
                    let movement = PlayerMoveEvent::new(client.clone(), position.to_server(), rotation.to_server());
                    player_move_events.send(movement);
                }
                ClientMessages::ConnectionInfo {
                    login,
                    version,
                    architecture,
                    rendering_device,
                } => {
                    let info =
                        PlayerConnectionInfoEvent::new(client.clone(), login, version, architecture, rendering_device);
                    connection_info_events.send(info);
                }
                ClientMessages::EditBlockRequest {
                    world_slug,
                    position,
                    new_block_info,
                } => {
                    let edit = EditBlockEvent::new(client.clone(), world_slug, position, new_block_info);
                    edit_block_events.send(edit);
                }
            }
        }
    }
}

#[allow(unused_mut)]
fn console_client_command_event(world: &mut World) {
    world.resource_scope(|world, mut clients: Mut<ClientsContainer>| {
        for (client_id, command) in CONSOLE_INPUT.1.try_iter() {
            let client = clients.get(&client_id).unwrap();
            CommandsHandler::execute_command(world, Box::new(client.clone()), &command);
        }
    });
}

fn handle_events_system(
    mut clients: ResMut<ClientsContainer>,
    network_container: Res<NetworkContainer>,

    mut connection_events: EventWriter<PlayerConnectionEvent>,
    mut disconnection_events: EventWriter<PlayerDisconnectEvent>,
) {
    let network = network_container.server_network.as_ref();

    for connection in network.drain_connections() {
        match connection {
            ConnectionMessages::Connect { connection } => {
                clients.add(connection.clone());
                let client = clients.get(&connection.get_client_id()).unwrap();
                connection_events.send(PlayerConnectionEvent::new(client.clone()));
            }
            ConnectionMessages::Disconnect { client_id, reason } => {
                let client = clients.get(&client_id).unwrap();
                disconnection_events.send(PlayerDisconnectEvent::new(client.clone(), reason));
            }
        }
    }
}
