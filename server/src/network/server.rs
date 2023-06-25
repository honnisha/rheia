use bevy_app::App;
use bevy_ecs::{
    prelude::EventReader,
    system::{ResMut, Resource, Res},
};
use bevy_renet::{
    renet::{
        transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
        DefaultChannel, RenetServer, ServerEvent,
    },
    transport::NetcodeServerPlugin,
    RenetServerPlugin,
};
use common::network::{connection_config, ClientMessages, PROTOCOL_ID};
use dashmap::DashMap;
use log::info;
use parking_lot::{MappedMutexGuard, Mutex, MutexGuard};
use std::{net::UdpSocket, time::SystemTime};

use crate::{console::console_handler::ConsoleHandler, ServerSettings};

use super::player_network::PlayerNetwork;

pub struct NetworkPlugin;

#[derive(Resource)]
pub struct Players {
    players: DashMap<u64, PlayerNetwork>,
}

impl Default for Players {
    fn default() -> Self {
        Players {
            players: DashMap::new(),
        }
    }
}

impl Players {
    pub fn add(&mut self, client_id: &u64) {
        self.players
            .insert(client_id.clone(), PlayerNetwork::new(client_id.clone()));
    }

    pub fn remove(&mut self, client_id: &u64) {
        self.players.remove(client_id);
    }

    fn get_mut(&self, key: &u64) -> dashmap::mapref::one::RefMut<'_, u64, PlayerNetwork> {
        self.players.get_mut(key).unwrap()
    }
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
    }
}

fn receive_message_system(mut server: ResMut<RenetServer>, players: Res<Players>) {
    // Send a text message for all clients
    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
            let command: ClientMessages = bincode::deserialize(&message).unwrap();
            match command {
                ClientMessages::ConsoleInput { command } => {
                    let player_info = players.get_mut(&client_id);
                    ConsoleHandler::execute_command(player_info.value(), &command);
                }
            }
        }
    }
}

fn handle_events_system(mut server_events: EventReader<ServerEvent>, mut players: ResMut<Players>) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                players.add(client_id);
                info!("Client {client_id} connected");
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                players.remove(client_id);
                info!("Client {client_id} disconnected: {reason}");
            }
        }
    }
}
