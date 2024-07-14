use flume::Drain;
use parking_lot::RwLockReadGuard;

use super::messages::{ServerMessages, ClientMessages, NetworkMessageType};

#[derive(Default, Clone)]
pub struct NetworkInfo {
    pub is_disconnected: bool,
    pub bytes_received_per_second: f64,
    pub bytes_received_per_sec: f64,
    pub bytes_sent_per_sec: f64,
    pub packet_loss: f64,
}

pub trait ClientNetwork: Sized {
    fn new(ip_port: String) -> Result<Self, String>;
    // fn step(&self, delta: Duration) -> bool;

    fn iter_server_messages(&self) -> Drain<ServerMessages>;
    fn iter_errors(&self) -> Drain<String>;

    fn is_connected(&self) -> bool;

    fn disconnect(&self);

    fn send_message(&self, message: &ClientMessages, message_type: NetworkMessageType);

    fn get_network_info(&self) -> RwLockReadGuard<NetworkInfo>;
}
