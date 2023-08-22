use std::sync::Arc;

use bevy::utils::HashMap;
use bevy_ecs::system::Resource;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use super::client_network::ClientNetwork;

pub type ClientCell = Arc<RwLock<ClientNetwork>>;
pub type ClientRef<'a> = RwLockReadGuard<'a, ClientNetwork>;
pub type ClientMut<'a> = RwLockWriteGuard<'a, ClientNetwork>;

#[derive(Resource)]
pub struct ClientsContainer {
    players: HashMap<u64, ClientCell>,
}

impl Default for ClientsContainer {
    fn default() -> Self {
        Self {
            players: HashMap::new(),
        }
    }
}

impl ClientsContainer {
    pub fn add(&mut self, client_id: &u64, login: String) {
        self.players.insert(
            client_id.clone(),
            Arc::new(RwLock::new(ClientNetwork::new(client_id.clone(), login))),
        );
    }

    pub fn remove(&mut self, client_id: &u64) {
        self.players.remove(client_id);
    }

    pub fn get(&self, key: &u64) -> ClientRef {
        self.players.get(key).unwrap().read()
    }

    pub fn get_mut<'a>(&self, key: &u64) -> ClientMut {
        match self.players.get(key) {
            Some(e) => e.write(),
            None => panic!("no player with client_id: {}", key),
        }
    }
}
