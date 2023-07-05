use std::sync::Arc;

use bevy::utils::HashMap;
use bevy_ecs::system::Resource;
use parking_lot::{RwLock, RwLockWriteGuard, RwLockReadGuard};

use super::player_network::PlayerNetwork;

pub type PlayerCell = Arc<RwLock<PlayerNetwork>>;
pub type PlayerRef<'a> = RwLockReadGuard<'a, PlayerNetwork>;
pub type PlayerMut<'a> = RwLockWriteGuard<'a, PlayerNetwork>;

#[derive(Resource)]
pub struct Players {
    players: HashMap<u64, PlayerCell>,
}

impl Default for Players {
    fn default() -> Self {
        Players {
            players: HashMap::new(),
        }
    }
}

impl Players {
    pub fn add(&mut self, client_id: &u64, login: String) {
        self.players.insert(
            client_id.clone(),
            Arc::new(RwLock::new(PlayerNetwork::new(client_id.clone(), login))),
        );
    }

    pub fn remove(&mut self, client_id: &u64) {
        self.players.remove(client_id);
    }

    pub fn get(&self, key: &u64) -> PlayerRef {
        self.players.get(key).unwrap().read()
    }

    pub fn get_mut<'a>(&self, key: &u64) -> PlayerMut {
        match self.players.get(key) {
            Some(e) => {
                e.write()
            },
            None => panic!("no player with client_id: {}", key),
        }
    }
}
