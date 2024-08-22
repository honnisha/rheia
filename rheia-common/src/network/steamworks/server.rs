use crate::network::{
    messages::{ClientMessages, NetworkMessageType, ServerMessages},
    server::{ConnectionMessages, ServerNetwork},
};
//use steamworks::*;

pub struct SteamworksServer {}

// https://github.com/lucaspoffo/renet/blob/master/renet_steam/src/server.rs

impl ServerNetwork for SteamworksServer {
    fn new(_ip_port: String) -> Self {
        Self {
        }
    }

    fn step(&self, _delta: std::time::Duration) -> bool {
        todo!()
    }

    fn iter_client_messages(&self) -> flume::Drain<(u64, ClientMessages)> {
        todo!()
    }

    fn iter_connections(&self) -> flume::Drain<ConnectionMessages> {
        todo!()
    }

    fn iter_errors(&self) -> flume::Drain<String> {
        todo!()
    }

    fn is_connected(&self, _client_id: u64) -> bool {
        todo!()
    }

    fn send_message(&self, _client_id: u64, _message: &ServerMessages, _message_type: NetworkMessageType) {
        todo!()
    }
}
