use bevy_ecs::prelude::EventReader;
use bincode::DefaultOptions;
use serde::{Deserialize, Serialize};

use bevy_app::{App, Plugin};
use network::{
    packet_length_serializer::LittleEndian, protocols::tcp::TcpProtocol, serializers::bincode::BincodeSerializer,
    server::{ServerPlugin, NewConnectionEvent, PacketReceiveEvent}, ServerConfig,
};

use crate::{ServerSettings, console_send};

#[derive(Serialize, Deserialize, Debug)]
enum ClientPacket {
    String(String),
}

#[derive(Serialize, Deserialize, Debug)]
enum ServerPacket {
    String(String),
}

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

        console_send(format!("Starting server on {}", ip_port));

        app.add_plugin(ServerPlugin::<Config>::bind(ip_port));
        app.add_system(new_connection_system);
        app.add_system(packet_receive_system);
        app.run();
    }
}

fn new_connection_system(mut events: EventReader<NewConnectionEvent<Config>>) {
    for event in events.iter() {
        event
            .connection
            .send(ServerPacket::String("Hello, Client!".to_string())).unwrap();
    }
}

fn packet_receive_system(mut events: EventReader<PacketReceiveEvent<Config>>) {
    for event in events.iter() {
        match &event.packet {
            ClientPacket::String(s) => println!("Got a message from a client: {}", s),
        }
        event
            .connection
            .send(ServerPacket::String("Hello, Client!".to_string())).unwrap();
    }
}
