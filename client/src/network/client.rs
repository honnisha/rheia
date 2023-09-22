use common::network::{client::ClientNetwork, renet::client::RenetClientNetwork};
use log::info;
use parking_lot::RwLock;
use std::sync::Arc;

pub type NetworkClientType = RenetClientNetwork;
pub type NetworkLockType = Arc<RwLock<NetworkClientType>>;

pub struct NetworkContainer {
    client_network: Arc<RwLock<NetworkClientType>>,
}

impl NetworkContainer {
    pub fn new(ip_port: String, login: String) -> Result<Self, String> {
        info!("Connecting to the server at {}", ip_port);
        let network = match NetworkClientType::new(ip_port, login) {
            Ok(n) => n,
            Err(e) => return Err(e),
        };
        network.spawn_network_thread();
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
            info!("{}", "Disconnected from the server");
            network.disconnect();
        }
    }
}
