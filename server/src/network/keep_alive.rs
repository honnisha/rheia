use std::marker::PhantomData;
use std::time::Duration;

use bevy::time::common_conditions::on_timer;
use bevy::time::{Time, Timer, TimerMode};
use bevy_app::App;
use bevy_ecs::prelude::EventReader;
use bevy_ecs::schedule::IntoSystemConfig;
use bevy_ecs::system::{Res, Resource};
use dashmap::DashMap;
use log::info;
use network::server::{NewConnectionEvent, PacketReceiveEvent};
use network::ClientPacket;
use network::{connection::ConnectionId, server::ServerConnections, ServerConfig, ServerPacket};

use super::Config;

#[derive(Resource)]
pub(crate) struct ServerKeepAliveMap<Config: ServerConfig> {
    map: DashMap<ConnectionId, Timer>,
    _marker: PhantomData<Config>,
}

impl<Config: ServerConfig> Default for ServerKeepAliveMap<Config> {
    fn default() -> Self {
        ServerKeepAliveMap {
            map: Default::default(),
            _marker: Default::default(),
        }
    }
}

impl<Config: ServerConfig> ServerKeepAliveMap<Config> {
    pub fn build(app: &mut App) {
        app.init_resource::<ServerKeepAliveMap<Config>>();
        app.add_system(server_send_keepalive.run_if(on_timer(Duration::from_secs_f32(0.5))));
        app.add_system(server_accept_new_connections);
        app.add_system(server_remove_timed_out_clients);
    }
}

fn server_accept_new_connections(
    mut event_reader: EventReader<NewConnectionEvent<Config>>,
    server_keepalive_map: Res<ServerKeepAliveMap<Config>>,
) {
    for event in event_reader.iter() {
        server_keepalive_map
            .map
            .insert(event.connection.id(), Timer::from_seconds(1.0, TimerMode::Once));
    }
}

fn server_send_keepalive(server: Res<ServerConnections<Config>>) {
    for client in server.iter() {
        let _ = client.send(ServerPacket::KeepAlive);
    }
}

fn server_remove_timed_out_clients(
    time: Res<Time>,

    lobby_connections: Res<ServerConnections<Config>>,
    server_keepalive_map: Res<ServerKeepAliveMap<Config>>,
    mut events: EventReader<PacketReceiveEvent<Config>>,
) {
    for event in events.iter() {
        if event.packet == ClientPacket::KeepAlive {
            server_keepalive_map
                .map
                .get_mut(&event.connection.id())
                .expect("keepalive not found")
                .reset();
        }
    }
    for connection in &**lobby_connections {
        if server_keepalive_map
            .map
            .get_mut(&connection.id())
            .expect("keepalive not found")
            .tick(time.delta())
            .just_finished()
        {
            connection.disconnect();
        }
    }
}
