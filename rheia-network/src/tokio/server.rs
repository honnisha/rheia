use std::time::Duration;

use crate::{
    messages::{ClientMessages, NetworkMessageType, ServerMessages},
    server::{ConnectionMessages, IServerNetwork},
};

pub struct TokioServer {
    channel_client_messages: (
        flume::Sender<(u64, ClientMessages)>,
        flume::Receiver<(u64, ClientMessages)>,
    ),
    channel_connections: (flume::Sender<ConnectionMessages>, flume::Receiver<ConnectionMessages>),
    channel_errors: (flume::Sender<String>, flume::Receiver<String>),
}

impl IServerNetwork for TokioServer {
    async fn new(_ip_port: String) -> Self {
        let result = Self {
            channel_client_messages: flume::unbounded(),
            channel_connections: flume::unbounded(),
            channel_errors: flume::unbounded(),
        };
        return result
    }

    async fn step(&self, _delta: Duration) {
        todo!()
    }

    fn drain_client_messages(&self) -> impl Iterator<Item = (u64, ClientMessages)> {
        self.channel_client_messages.1.drain()
    }

    fn drain_connections(&self) -> impl Iterator<Item = ConnectionMessages> {
        self.channel_connections.1.drain()
    }

    fn drain_errors(&self) -> impl Iterator<Item = String> {
        self.channel_errors.1.drain()
    }

    fn is_connected(&self, _client_id: u64) -> bool {
        todo!()
    }

    fn send_message(&self, _client_id: u64, _message_type: NetworkMessageType, _message: &ServerMessages) {
        todo!()
    }
}
