use network::client::IClientNetwork;
use network::NetworkClient;
use parking_lot::RwLock;
use std::sync::Arc;

pub type NetworkLockType = Arc<RwLock<NetworkClient>>;

pub struct NetworkContainer {
    client_network: Arc<RwLock<NetworkClient>>,
}

impl NetworkContainer {
    pub fn new(ip_port: String) -> Result<Self, String> {
        log::info!(target: "network", "Connecting to the server at {}", ip_port);

        let io_loop = tokio::runtime::Runtime::new().unwrap();
        let result = io_loop.block_on(async { NetworkClient::new(ip_port) });

        let network = match io_loop.block_on(result) {
            Ok(n) => n,
            Err(e) => return Err(e),
        };
        Ok(Self {
            client_network: Arc::new(RwLock::new(network)),
        })
    }

    pub fn get_network_lock(&self) -> Arc<RwLock<NetworkClient>> {
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
