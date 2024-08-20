use std::{sync::Arc};

use bevy::utils::{HashMap};
use bevy_ecs::system::Resource;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use super::client_network::ClientNetwork;

pub type ClientCell = Arc<RwLock<ClientNetwork>>;
pub type ClientRef<'a> = RwLockReadGuard<'a, ClientNetwork>;
pub type _ClientMut<'a> = RwLockWriteGuard<'a, ClientNetwork>;

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
    pub fn iter(&self) -> bevy::utils::hashbrown::hash_map::Iter<'_, u64, ClientCell>  {
        self.players.iter()
    }

    pub fn add(&mut self, client_id: u64, ip: String) {
        self.players.insert(
            client_id.clone(),
            Arc::new(RwLock::new(ClientNetwork::new(client_id, ip))),
        );
    }

    pub fn remove(&mut self, client_id: &u64) {
        self.players.remove(client_id);
    }

    pub fn get(&self, key: &u64) -> Option<&ClientCell> {
        self.players.get(key)
    }
}
