use common::network::client::ClientNetwork;
use parking_lot::RwLock;
use std::sync::Arc;

use crate::main_scene::NetworkClientType;

pub type NetworkLockType = Arc<RwLock<NetworkClientType>>;

pub struct NetworkContainer {
    client_network: Arc<RwLock<NetworkClientType>>,
}

impl NetworkContainer {
    pub fn new(ip_port: String) -> Result<Self, String> {
        log::info!(target: "network", "Connecting to the server at {}", ip_port);
        let network = match NetworkClientType::new(ip_port) {
            Ok(n) => n,
            Err(e) => return Err(e),
        };
        Ok(Self {
            client_network: Arc::new(RwLock::new(network)),
        })
    }

    pub fn get_network_lock(&self) -> Arc<RwLock<NetworkClientType>> {
        self.client_network.clone()
    }

    pub fn disconnect(&self) {
        let network = self.client_network.read();

        if network.is_connected() {
            log::info!(target: "network", "Disconnected from the server");
            network.disconnect();
        }
    }
}
