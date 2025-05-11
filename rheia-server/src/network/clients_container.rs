use ahash::AHashMap;
use bevy_ecs::resource::Resource;
use network::NetworkServerConnection;

use super::client_network::ClientNetwork;

#[derive(Resource)]
pub struct ClientsContainer {
    players: AHashMap<u64, ClientNetwork>,
}

impl Default for ClientsContainer {
    fn default() -> Self {
        Self {
            players: AHashMap::new(),
        }
    }
}

impl ClientsContainer {
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, u64, ClientNetwork> {
        self.players.iter()
    }

    pub fn add(&mut self, connection: NetworkServerConnection) {
        let network = ClientNetwork::new(connection);
        self.players.insert(network.get_client_id(), network);
    }

    pub fn remove(&mut self, client_id: &u64) {
        self.players.remove(client_id);
    }

    pub fn disconnect_all(&mut self, message: Option<String>) {
        for (_client_id, mut client) in self.players.drain() {
            client.send_disconnect(message.clone());
        }
    }

    pub fn get(&self, key: &u64) -> Option<&ClientNetwork> {
        self.players.get(key)
    }
}
