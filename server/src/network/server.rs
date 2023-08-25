use bevy::prelude::{Event, IntoSystemConfigs};
use bevy::time::Time;
use bevy_app::{App, Update};
use bevy_ecs::change_detection::Mut;
use bevy_ecs::{
    prelude::{EventReader, EventWriter, Events},
    system::{Res, ResMut, Resource},
    world::World,
};
use common::network::messages::{ClientMessages, ServerMessages};
use common::network::{channels::ClientChannel, channels::ServerChannel, connection_config, login::Login, PROTOCOL_ID};
use flume::{Receiver, Sender};
use lazy_static::lazy_static;
use log::error;
use log::info;
use renet::{
    transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
    RenetServer, ServerEvent,
};
use std::{
    net::UdpSocket,
    sync::{Arc, RwLock},
    time::SystemTime,
};

use crate::entities::entity::Rotation;
use crate::network::clients_container::ClientsContainer;
use crate::{
    client_resources::resources_manager::ResourceManager,
    console::commands_executer::CommandsHandler,
    entities::entity::Position,
    events::{
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
    channel: u8,
    bytes: Vec<u8>,
}

lazy_static! {
    static ref CONSOLE_INPUT: (Sender<(u64, String)>, Receiver<(u64, String)>) = flume::unbounded();
    // static ref CLIENT_MESSAGES_OUTPUT: Arc<RwLock<Events<SendClientMessageEvent>>> = Arc::new(RwLock::new(Events::<SendClientMessageEvent>::default()));
    static ref CLIENT_MESSAGES_OUTPUT: (Sender<SendClientMessageEvent>, Receiver<SendClientMessageEvent>) = flume::unbounded();
}

pub type ServerLock = Arc<RwLock<RenetServer>>;
pub type TransferLock = Arc<RwLock<NetcodeServerTransport>>;

#[derive(Resource)]
pub struct NetworkContainer {
    pub server: ServerLock,
    pub transport: TransferLock,
}

impl NetworkPlugin {
    pub fn build(app: &mut App) {
        let server_settings = app.world.get_resource::<ServerSettings>().unwrap();
        let ip_port = format!("{}:{}", server_settings.get_args().ip, server_settings.get_args().port);

        info!("Starting server on {}", ip_port);

        app.init_resource::<Events<ServerEvent>>();

        let server = RenetServer::new(connection_config());

        let public_addr = ip_port.parse().unwrap();
        let socket = UdpSocket::bind(public_addr).unwrap();
        let server_config = ServerConfig {
            max_clients: 64,
            protocol_id: PROTOCOL_ID,
            public_addr,
            authentication: ServerAuthentication::Unsecure,
        };
        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

        let transport = NetcodeServerTransport::new(current_time, server_config, socket).unwrap();

        app.insert_resource(NetworkContainer {
            server: Arc::new(RwLock::new(server)),
            transport: Arc::new(RwLock::new(transport)),
        });

        app.insert_resource(ClientsContainer::default());

        app.add_systems(Update, receive_message_system);
        app.add_systems(Update, handle_events_system);

        app.add_systems(Update, console_client_command_event);

        app.add_event::<PlayerConnectionEvent>();
        app.add_systems(Update, on_connection.after(handle_events_system));

        app.add_event::<PlayerDisconnectEvent>();
        app.add_systems(Update, on_disconnect.after(handle_events_system));

        app.add_event::<PlayerMoveEvent>();
        app.add_systems(Update, on_player_move.after(handle_events_system));

        app.add_event::<SendClientMessageEvent>();
        app.add_systems(Update, send_client_messages.after(on_disconnect));
    }

    pub(crate) fn send_console_output(client_id: u64, message: String) {
        let input = ServerMessages::ConsoleOutput { message: message };
        let encoded = bincode::serialize(&input).unwrap();
        NetworkPlugin::send_static_message(client_id, ServerChannel::Reliable.into(), encoded)
    }

    pub(crate) fn send_resources(client_id: &u64, resources_manager: &Res<ResourceManager>) {
        for resource in resources_manager.get_resources().values() {
            let input = ServerMessages::Resource {
                slug: resource.get_slug().clone(),
                scripts: resource.get_client_scripts().clone(),
            };
            let encoded = bincode::serialize(&input).unwrap();
            NetworkPlugin::send_static_message(client_id.clone(), ServerChannel::Reliable.into(), encoded)
        }
    }

    pub(crate) fn send_static_message(client_id: u64, channel: u8, bytes: Vec<u8>) {
        CLIENT_MESSAGES_OUTPUT
            .0
            .send(SendClientMessageEvent {
                client_id: client_id,
                bytes: bytes,
                channel: channel,
            })
            .unwrap();
    }
}

fn receive_message_system(
    network_container: Res<NetworkContainer>,
    time: Res<Time>,
    mut server_events: EventWriter<ServerEvent>,
    mut player_move_events: EventWriter<PlayerMoveEvent>,
) {
    let mut server = network_container.server.write().expect("poisoned");
    let mut transport = network_container.transport.write().expect("poisoned");
    server.update(time.delta());

    if let Err(e) = transport.update(time.delta(), &mut server) {
        error!("Transport error: {}", e.to_string());
    }

    for client_id in server.clients_id().into_iter() {
        while let Some(client_message) = server.receive_message(client_id, ClientChannel::Reliable) {
            let decoded: ClientMessages = match bincode::deserialize(&client_message) {
                Ok(d) => d,
                Err(e) => {
                    error!("Decode client reliable message error: {}", e);
                    continue;
                }
            };
            match decoded {
                ClientMessages::ConsoleInput { command } => {
                    CONSOLE_INPUT.0.send((client_id.clone(), command)).unwrap();
                }
                _ => panic!("unsupported message"),
            }
        }
        while let Some(client_message) = server.receive_message(client_id, ClientChannel::Unreliable) {
            let decoded: ClientMessages = match bincode::deserialize(&client_message) {
                Ok(d) => d,
                Err(e) => {
                    error!("Decode client unreliable message error: {}", e);
                    continue;
                }
            };
            match decoded {
                ClientMessages::PlayerMove { position, yaw, pitch } => {
                    player_move_events.send(PlayerMoveEvent::new(
                        client_id.clone(),
                        Position::from_array(position),
                        Rotation::new(pitch, yaw),
                    ));
                }
                _ => panic!("unsupported message"),
            }
        }
    }

    while let Some(event) = server.get_event() {
        server_events.send(event);
    }

    transport.send_packets(&mut server);
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
    mut server_events: EventReader<ServerEvent>,
    mut clients: ResMut<ClientsContainer>,
    network_container: Res<NetworkContainer>,

    mut connection_events: EventWriter<PlayerConnectionEvent>,
    mut disconnection_events: EventWriter<PlayerDisconnectEvent>,
) {
    let transport = network_container.transport.read().expect("poisoned");

    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                let user_data = transport.user_data(client_id.clone()).unwrap();
                let login = Login::from_user_data(&user_data).0;
                clients.add(client_id, login.clone());
                connection_events.send(PlayerConnectionEvent::new(client_id.clone()));
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                disconnection_events.send(PlayerDisconnectEvent::new(client_id.clone(), reason.clone()));
            }
        }
    }
}

fn send_client_messages(network_container: Res<NetworkContainer>) {
    let mut server = network_container.server.write().expect("poisoned");
    for event in CLIENT_MESSAGES_OUTPUT.1.drain() {
        server.send_message(event.client_id, event.channel, event.bytes);
    }
}
