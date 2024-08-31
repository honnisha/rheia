use std::thread;

use bevy::prelude::{Event, IntoSystemConfigs};
use bevy::time::Time;
use bevy_app::{App, Update};
use bevy_ecs::change_detection::Mut;
use bevy_ecs::{
    prelude::EventWriter,
    system::{Res, ResMut, Resource},
    world::World,
};
use common::network::messages::{ClientMessages, NetworkMessageType, ServerMessages};
use common::network::server::{ConnectionMessages, ServerNetwork};
use common::network::NetworkServer;
use flume::{Receiver, Sender};
use lazy_static::lazy_static;

use super::events::{
    on_connection::{on_connection, PlayerConnectionEvent},
    on_connection_info::{on_connection_info, PlayerConnectionInfoEvent},
    on_disconnect::{on_disconnect, PlayerDisconnectEvent},
    on_player_move::{on_player_move, PlayerMoveEvent},
};
use crate::entities::entity::{IntoServerPosition, IntoServerRotation};
use crate::network::chunks_sender::send_chunks;
use crate::network::client_network::ClientNetwork;
use crate::network::clients_container::ClientsContainer;
use crate::network::sync_entities::{sync_player_spawn, PlayerSpawnEvent};
use crate::{
    client_resources::resources_manager::ResourceManager, console::commands_executer::CommandsHandler, ServerSettings,
};

const MIN_TICK_TIME: std::time::Duration = std::time::Duration::from_millis(50);

pub struct NetworkPlugin;

#[derive(Event)]
pub struct SendClientMessageEvent {
    client_id: u64,
    message_type: NetworkMessageType,
    message: ServerMessages,
}

impl SendClientMessageEvent {
    pub fn new(client_id: u64, message_type: NetworkMessageType, message: ServerMessages) -> Self {
        Self {
            client_id,
            message_type,
            message,
        }
    }
}

lazy_static! {
    static ref CONSOLE_INPUT: (Sender<(u64, String)>, Receiver<(u64, String)>) = flume::unbounded();
}

#[derive(Resource)]
pub struct NetworkContainer {
    server_network: Box<NetworkServer>,
}

impl NetworkContainer {
    pub fn new(ip_port: String) -> Self {
        Self {
            server_network: Box::new(NetworkServer::new(ip_port)),
        }
    }

    pub fn is_connected(&self, client_id: &u64) -> bool {
        let network = self.server_network.as_ref();
        network.is_connected(*client_id)
    }
}

impl NetworkPlugin {
    pub fn build(app: &mut App) {
        let server_settings = app.world().get_resource::<ServerSettings>().unwrap();
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

        app.add_event::<SendClientMessageEvent>();
        app.add_systems(Update, send_client_messages.after(on_disconnect));

        app.add_event::<PlayerSpawnEvent>();
        app.add_systems(Update, sync_player_spawn);
    }

    pub(crate) fn send_console_output(client: &ClientNetwork, message: String) {
        let input = ServerMessages::ConsoleOutput { message: message };
        client.send_message(NetworkMessageType::ReliableOrdered, input);
    }

    pub(crate) fn send_resources(client: &ClientNetwork, resources_manager: &Res<ResourceManager>) {
        for resource in resources_manager.get_resources().values() {
            let input = ServerMessages::Resource {
                slug: resource.get_slug().clone(),
                scripts: resource.get_client_scripts().clone(),
            };
            client.send_message(NetworkMessageType::ReliableOrdered, input);
        }
    }
}

fn receive_message_system(
    network_container: Res<NetworkContainer>,
    time: Res<Time>,
    clients: Res<ClientsContainer>,
    mut connection_info_events: EventWriter<PlayerConnectionInfoEvent>,
    mut player_move_events: EventWriter<PlayerMoveEvent>,
) {
    #[cfg(feature = "trace")]
    let _span = bevy_utils::tracing::info_span!("receive_message_system").entered();

    // Sleep if the tick rate is less than the minimum tick rate
    if time.delta() < MIN_TICK_TIME {
        thread::sleep(MIN_TICK_TIME - time.delta());
    }

    let network = network_container.server_network.as_ref();
    network.step(time.delta());

    for message in network.drain_errors() {
        log::error!(target: "network", "Network error: {}", message);
    }

    for (client_id, decoded) in network.drain_client_messages() {
        let client = clients.get(&client_id).unwrap();
        match decoded {
            ClientMessages::ConsoleInput { command } => {
                CONSOLE_INPUT.0.send((client_id, command)).unwrap();
            }
            ClientMessages::ChunkRecieved { chunk_positions } => {
                client.read().mark_chunks_as_recieved(chunk_positions);
            }
            ClientMessages::PlayerMove { position, rotation } => {
                let movement = PlayerMoveEvent::new(client.clone(), position.to_server(), rotation.to_server());
                player_move_events.send(movement);
            }
            ClientMessages::ConnectionInfo { login } => {
                let info = PlayerConnectionInfoEvent::new(client.clone(), login);
                connection_info_events.send(info);
            }
        }
    }
}

#[allow(unused_mut)]
fn console_client_command_event(world: &mut World) {
    world.resource_scope(|world, mut clients: Mut<ClientsContainer>| {
        for (client_id, command) in CONSOLE_INPUT.1.try_iter() {
            let client = clients.get(&client_id).unwrap().read();
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
            ConnectionMessages::Connect { client_id, ip } => {
                clients.add(client_id.clone(), ip.clone());
                let client = clients.get(&client_id).unwrap();
                connection_events.send(PlayerConnectionEvent::new(client.clone()));
            }
            ConnectionMessages::Disconnect { client_id, reason } => {
                let client = clients.get(&client_id).unwrap();
                disconnection_events.send(PlayerDisconnectEvent::new(client.clone(), reason));
            }
        }
    }
}

fn send_client_messages(network_container: Res<NetworkContainer>, clients: Res<ClientsContainer>) {
    let network = network_container.server_network.as_ref();

    for (client_id, client_lock) in clients.iter() {
        if !network_container.is_connected(&client_id) {
            continue;
        }

        let client = client_lock.read();
        for message in client.drain_client_messages() {
            network.send_message(message.client_id, &message.message, message.message_type);
        }
    }
}
