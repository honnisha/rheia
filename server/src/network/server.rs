use bevy_app::App;
use bevy_ecs::{
    prelude::{EventReader, EventWriter},
    system::{Res, ResMut, Resource},
    world::World,
};
use bevy_renet::{
    renet::{
        transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
        RenetServer, ServerEvent,
    },
    transport::NetcodeServerPlugin,
    RenetServerPlugin,
};
use common::network::{
    connection_config, ClientChannel, ClientMessages, Login, ServerChannel, ServerMessages, PROTOCOL_ID,
};
use dashmap::DashMap;
use flume::{Receiver, Sender};
use lazy_static::lazy_static;
use log::error;
use log::info;
use std::{net::UdpSocket, time::SystemTime};

use crate::{
    client_resources::resources_manager::ResourceManager, console::commands_executer::CommandsHandler,
    events::{connection::{PlayerConnectionEvent, on_connection}, disconnect::{PlayerDisconnectEvent, on_disconnect}}, ServerSettings,
};

use super::player_network::PlayerNetwork;

pub struct NetworkPlugin;

#[derive(Resource)]
pub struct Players {
    players: DashMap<u64, Box<PlayerNetwork>>,
}

impl Default for Players {
    fn default() -> Self {
        Players {
            players: DashMap::new(),
        }
    }
}

impl Players {
    pub fn add(&mut self, client_id: &u64, login: String) {
        self.players.insert(
            client_id.clone(),
            Box::new(PlayerNetwork::new(client_id.clone(), login)),
        );
    }

    pub fn remove(&mut self, client_id: &u64) {
        self.players.remove(client_id);
    }

    fn get(&self, key: &u64) -> dashmap::mapref::one::Ref<'_, u64, Box<PlayerNetwork>> {
        self.players.get(key).unwrap()
    }
}

lazy_static! {
    static ref CONSOLE_OUTPUT: (Sender<(u64, String)>, Receiver<(u64, String)>) = flume::unbounded();
    static ref CONSOLE_INPUT: (Sender<(u64, String)>, Receiver<(u64, String)>) = flume::unbounded();
}

impl NetworkPlugin {
    pub fn build(app: &mut App) {
        let server_settings = app.world.get_resource::<ServerSettings>().unwrap();
        let ip_port = format!("{}:{}", server_settings.get_args().ip, server_settings.get_args().port);

        info!("Starting server on {}", ip_port);

        app.add_plugin(RenetServerPlugin);
        app.add_plugin(NetcodeServerPlugin);

        let server = RenetServer::new(connection_config());
        app.insert_resource(server);

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

        app.insert_resource(transport);
        app.insert_resource(Players::default());

        app.add_system(receive_message_system);
        app.add_system(handle_events_system);
        app.add_system(send_messages);

        app.add_system(console_client_command_event);

        app.add_event::<PlayerConnectionEvent>();
        app.add_system(on_connection);

        app.add_event::<PlayerDisconnectEvent>();
        app.add_system(on_disconnect);
    }

    pub(crate) fn send_console_output(client_id: u64, message: String) {
        CONSOLE_OUTPUT.0.send((client_id, message)).unwrap();
    }

    pub(crate) fn send_resources(
        player_network: Box<PlayerNetwork>,
        resources_manager: &Res<ResourceManager>,
        server: &mut ResMut<RenetServer>,
    ) {
        for resource in resources_manager.get_resources().values() {
            let input = ServerMessages::Resource {
                slug: resource.get_slug().clone(),
                scripts: resource.get_client_scripts().clone(),
            };
            let encoded = bincode::serialize(&input).unwrap();
            server.send_message(player_network.get_client_id().clone(), ServerChannel::Messages, encoded);
        }
    }
}

fn send_messages(mut server: ResMut<RenetServer>) {
    for (client_id, message) in CONSOLE_OUTPUT.1.try_iter() {
        let input = ServerMessages::ConsoleOutput { message: message };
        let encoded = bincode::serialize(&input).unwrap();
        server.send_message(client_id, ServerChannel::Messages, encoded);
    }
}

fn receive_message_system(mut server: ResMut<RenetServer>) {
    for client_id in server.clients_id().into_iter() {
        while let Some(client_message) = server.receive_message(client_id, ClientChannel::Messages) {
            let decoded: ClientMessages = match bincode::deserialize(&client_message) {
                Ok(d) => d,
                Err(e) => {
                    error!("Decode client message error: {}", e);
                    continue;
                }
            };
            match decoded {
                ClientMessages::ConsoleInput { command } => {
                    CONSOLE_INPUT.0.send((client_id.clone(), command)).unwrap();
                }
            }
        }
    }
}

fn console_client_command_event(world: &mut World) {
    for (client_id, command) in CONSOLE_INPUT.1.try_iter() {
        let players = world.resource::<Players>();
        let player_info = players.get(&client_id).value().clone();
        CommandsHandler::execute_command(world, &*player_info, &command);
    }
}

fn handle_events_system(
    mut server_events: EventReader<ServerEvent>,
    mut players: ResMut<Players>,
    transport: Res<NetcodeServerTransport>,

    mut connection_events: EventWriter<PlayerConnectionEvent>,
    mut disconnection_events: EventWriter<PlayerDisconnectEvent>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                let user_data = transport.user_data(client_id.clone()).unwrap();
                let login = Login::from_user_data(&user_data).0;
                players.add(client_id, login.clone());
                connection_events.send(
                    PlayerConnectionEvent::new(players.get(client_id).value().clone())
                );
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                disconnection_events.send(
                    PlayerDisconnectEvent::new(
                        players.get(client_id).value().clone(),
                        reason.clone(),
                    )
                );
                players.remove(client_id);
            }
        }
    }
}
