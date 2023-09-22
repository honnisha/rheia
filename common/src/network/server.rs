use std::time::Duration;

use flume::Drain;

use super::messages::{ClientMessages, ServerMessages, NetworkMessageType};

pub trait ServerNetwork {
    fn new(ip_port: String) -> Self;
    fn step(&self, delta: Duration) -> bool;

    fn iter_client_messages(&self) -> Drain<(u64, ClientMessages)>;
    fn iter_connections(&self) -> Drain<ConnectionMessages>;
    fn iter_errors(&self) -> Drain<String>;

    fn is_connected(&self, client_id: u64) -> bool;
    fn send_message(&self, client_id: u64, message: &ServerMessages, message_type: NetworkMessageType);
}

pub enum ConnectionMessages {
    Connect{
        client_id: u64,
        login: String,
    },
    Disconnect{
        client_id: u64,
        reason: String,
    },
}
