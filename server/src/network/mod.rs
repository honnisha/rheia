use std::{time::Duration, thread};

use bevy_ecs::{prelude::EventReader, schedule::IntoSystemConfig};
use bincode::DefaultOptions;
use log::info;

use bevy_app::{App, AppExit, CoreSet};
use network::{
    packet_length_serializer::LittleEndian,
    protocols::tcp::TcpProtocol,
    serializers::bincode::BincodeSerializer,
    server::{NewConnectionEvent, PacketReceiveEvent, ServerPlugin},
    ServerConfig, connection::MaxPacketSize, ClientPacket, ServerPacket,
};

use crate::{ServerSettings};

struct Config;

impl ServerConfig for Config {
    type ClientPacket = ClientPacket;
    type ServerPacket = ServerPacket;
    type Protocol = TcpProtocol;
    type Serializer = BincodeSerializer<DefaultOptions>;
    type LengthSerializer = LittleEndian<u32>;
}

pub struct NetworkPlugin;

impl NetworkPlugin {
    pub fn build(app: &mut App) {
        let server_settings = app.world.get_resource::<ServerSettings>().unwrap();
        let ip_port = format!("{}:{}", server_settings.get_args().ip, server_settings.get_args().port);

        info!("Starting server on {}", ip_port);

        let max_packet_size = server_settings.get_args().max_packet_size;
        app.insert_resource(MaxPacketSize(max_packet_size.clone()));

        app.add_plugin(ServerPlugin::<Config>::bind(ip_port));
        app.add_system(new_connection_system);
        app.add_system(packet_receive_system);
        app.add_system(stop_server.in_base_set(CoreSet::PostUpdate));
    }
}

fn stop_server(mut exit: EventReader<AppExit>) {
    for _e in exit.iter() {
        info!("Stopping the server");
        thread::sleep(Duration::from_millis(50));
    }
}

fn new_connection_system(mut events: EventReader<NewConnectionEvent<Config>>) {
    for event in events.iter() {
        event
            .connection
            .send(ServerPacket::String("Hello, Client!".to_string()))
            .unwrap();
    }
}

fn packet_receive_system(mut events: EventReader<PacketReceiveEvent<Config>>) {
    for event in events.iter() {
        match &event.packet {
            ClientPacket::String(s) => println!("Got a message from a client: {}", s),
        }
        event
            .connection
            .send(ServerPacket::String("Hello, Client!".to_string()))
            .unwrap();
    }
}
