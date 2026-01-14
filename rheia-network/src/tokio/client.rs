use crate::{
    client::{IClientNetwork, NetworkInfo},
    messages::{ClientMessages, NetworkMessageType, ServerMessages},
};

pub struct TokioClient {}

impl IClientNetwork for TokioClient {
    async fn new(_ip_port: String) -> Result<Self, String> {
        todo!()
    }

    async fn step(&self, _delta: std::time::Duration) -> bool {
        todo!()
    }

    fn iter_server_messages(&self) -> flume::Drain<'_, ServerMessages> {
        todo!()
    }

    fn iter_errors(&self) -> flume::Drain<'_, String> {
        todo!()
    }

    fn is_connected(&self) -> bool {
        todo!()
    }

    fn disconnect(&self) {
        todo!()
    }

    fn send_message(&self, _message_type: NetworkMessageType, _message: &ClientMessages) {
        todo!()
    }

    fn get_network_info(&self) -> parking_lot::RwLockReadGuard<'_, NetworkInfo> {
        todo!()
    }
}
