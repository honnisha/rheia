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
use common::network::renet::server::RenetServerNetwork;
use common::network::server::{ConnectionMessages, ServerNetwork};
use flume::{Receiver, Sender};
use lazy_static::lazy_static;
use log::error;
use log::info;
use std::borrow::Borrow;

use crate::entities::entity::{Position, Rotation};
use crate::network::chunks_sender::send_chunks;
use crate::network::clients_container::ClientsContainer;
use crate::{
    client_resources::resources_manager::ResourceManager,
    console::commands_executer::CommandsHandler,
    events::{
        on_chunk_recieved::{on_chunk_recieved, ChunkRecievedEvent},
        on_connection::{on_connection, PlayerConnectionEvent},
        on_disconnect::{on_disconnect, PlayerDisconnectEvent},
        on_player_move::{on_player_move, PlayerMoveEvent},
    },
    ServerSettings,
};

pub struct NetworkPlugin;

#[derive(Event)]
pub(crate) struct SendClientMessageEvent {
    client_id: u64,
    message_type: NetworkMessageType,
    message: ServerMessages,
}

lazy_static! {
    static ref CONSOLE_INPUT: (Sender<(u64, String)>, Receiver<(u64, String)>) = flume::unbounded();
    // static ref CLIENT_MESSAGES_OUTPUT: Arc<RwLock<Events<SendClientMessageEvent>>> = Arc::new(RwLock::new(Events::<SendClientMessageEvent>::default()));
    static ref CLIENT_MESSAGES_OUTPUT: (Sender<SendClientMessageEvent>, Receiver<SendClientMessageEvent>) = flume::unbounded();
}

pub type NetworkServerType = RenetServerNetwork;

#[derive(Resource)]
pub struct NetworkContainer {
    server_network: Box<NetworkServerType>,
}

impl NetworkContainer {
    pub fn new(ip_port: String) -> Self {
        Self {
            server_network: Box::new(NetworkServerType::new(ip_port)),
        }
    }

    pub fn is_connected(&self, client_id: &u64) -> bool {
        let network = self.server_network.as_ref().borrow();
        network.is_connected(*client_id)
    }
}

impl NetworkPlugin {
    pub fn build(app: &mut App) {
        let server_settings = app.world.get_resource::<ServerSettings>().unwrap();
        let ip_port = format!("{}:{}", server_settings.get_args().ip, server_settings.get_args().port);

        info!("Starting server on {}", ip_port);

        app.insert_resource(NetworkContainer::new(ip_port));
        app.insert_resource(ClientsContainer::default());

        app.add_systems(Update, receive_message_system);
        app.add_systems(Update, handle_events_system);
        app.add_systems(Update, send_chunks.after(handle_events_system));

        app.add_systems(Update, console_client_command_event);

        app.add_event::<PlayerConnectionEvent>();
        app.add_systems(Update, on_connection.after(handle_events_system));

        app.add_event::<PlayerDisconnectEvent>();
        app.add_systems(Update, on_disconnect.after(handle_events_system));

        app.add_event::<PlayerMoveEvent>();
        app.add_systems(Update, on_player_move.after(handle_events_system));

        app.add_event::<ChunkRecievedEvent>();
        app.add_systems(Update, on_chunk_recieved.after(handle_events_system));

        app.add_event::<SendClientMessageEvent>();
        app.add_systems(Update, send_client_messages.after(on_disconnect));
    }

    pub(crate) fn send_console_output(client_id: u64, message: String) {
        let input = ServerMessages::ConsoleOutput { message: message };
        NetworkPlugin::send_static_message(client_id, NetworkMessageType::Message, input);
    }

    pub(crate) fn send_resources(client_id: &u64, resources_manager: &Res<ResourceManager>) {
        for resource in resources_manager.get_resources().values() {
            let input = ServerMessages::Resource {
                slug: resource.get_slug().clone(),
                scripts: resource.get_client_scripts().clone(),
            };
            NetworkPlugin::send_static_message(client_id.clone(), NetworkMessageType::Message, input);
        }
    }

    pub(crate) fn send_static_message(client_id: u64, message_type: NetworkMessageType, message: ServerMessages) {
        let msg = SendClientMessageEvent {
            client_id,
            message_type,
            message,
        };
        CLIENT_MESSAGES_OUTPUT.0.send(msg).unwrap();
    }
}

fn receive_message_system(
    network_container: Res<NetworkContainer>,
    time: Res<Time>,
    mut player_move_events: EventWriter<PlayerMoveEvent>,
    mut chunk_recieved_events: EventWriter<ChunkRecievedEvent>,
) {
    if time.delta() > std::time::Duration::from_millis(100) {
        println!("receive_message_system delay: {:.2?}", time.delta());
    }
    let network = network_container.server_network.as_ref().borrow();
    network.step(time.delta());

    for message in network.iter_errors() {
        error!("Network error: {}", message);
    }

    for (client_id, decoded) in network.iter_client_messages() {
        match decoded {
            ClientMessages::ConsoleInput { command } => {
                CONSOLE_INPUT.0.send((client_id.clone(), command)).unwrap();
            }
            ClientMessages::ChunkRecieved { chunk_positions } => {
                chunk_recieved_events.send(ChunkRecievedEvent::new(client_id.clone(), chunk_positions));
            }
            ClientMessages::PlayerMove { position, yaw, pitch } => {
                player_move_events.send(PlayerMoveEvent::new(
                    client_id.clone(),
                    Position::from_array(position),
                    Rotation::new(pitch, yaw),
                ));
            }
        }
    }
}

#[allow(unused_mut)]
fn console_client_command_event(world: &mut World) {
    world.resource_scope(|world, mut clients: Mut<ClientsContainer>| {
        for (client_id, command) in CONSOLE_INPUT.1.try_iter() {
            let client = clients.get(&client_id);
            CommandsHandler::execute_command(world, &client.to_owned(), &command);
        }
    });
}

fn handle_events_system(
    mut clients: ResMut<ClientsContainer>,
    network_container: Res<NetworkContainer>,

    mut connection_events: EventWriter<PlayerConnectionEvent>,
    mut disconnection_events: EventWriter<PlayerDisconnectEvent>,
) {
    let network = network_container.server_network.as_ref().borrow();

    for connection in network.iter_connections() {
        match connection {
            ConnectionMessages::Connect { client_id, login } => {
                clients.add(&client_id, login);
                connection_events.send(PlayerConnectionEvent::new(client_id));
            }
            ConnectionMessages::Disconnect { client_id, reason } => {
                disconnection_events.send(PlayerDisconnectEvent::new(client_id, reason));
            }
        }
    }
}

fn send_client_messages(network_container: Res<NetworkContainer>) {
    let network = network_container.server_network.as_ref().borrow();

    for event in CLIENT_MESSAGES_OUTPUT.1.drain() {
        network.send_message(event.client_id, &event.message, event.message_type);
    }
}
