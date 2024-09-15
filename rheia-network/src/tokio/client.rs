use crate::{
    client::{IClientNetwork, NetworkInfo},
    messages::{ClientMessages, NetworkMessageType, ServerMessages},
};

pub struct TokioClient {}

impl IClientNetwork for TokioClient {
    fn new(_ip_port: String) -> Result<Self, String> {
        todo!()
    }

    fn iter_server_messages(&self) -> flume::Drain<ServerMessages> {
        todo!()
    }

    fn iter_errors(&self) -> flume::Drain<String> {
        todo!()
    }

    fn is_connected(&self) -> bool {
        todo!()
    }

    fn disconnect(&self) {
        todo!()
    }

    fn send_message(&self, _message: &ClientMessages, _message_type: NetworkMessageType) {
        todo!()
    }

    fn get_network_info(&self) -> parking_lot::RwLockReadGuard<NetworkInfo> {
        todo!()
    }
}
