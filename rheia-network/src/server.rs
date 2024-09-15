use std::time::Duration;

use super::messages::{ClientMessages, NetworkMessageType, ServerMessages};

pub trait IServerNetwork {
    fn new(ip_port: String) -> Self;
    fn step(&self, delta: Duration) -> bool;

    fn drain_client_messages(&self) -> impl Iterator<Item = (u64, ClientMessages)>;
    fn drain_connections(&self) -> impl Iterator<Item = ConnectionMessages>;
    fn drain_errors(&self) -> impl Iterator<Item = String>;

    fn is_connected(&self, client_id: u64) -> bool;
    fn send_message(&self, client_id: u64, message: &ServerMessages, message_type: NetworkMessageType);
}

pub enum ConnectionMessages {
    Connect { client_id: u64, ip: String },
    Disconnect { client_id: u64, reason: String },
}
