use std::collections::HashMap;

use super::world_manager::WorldManager;

pub struct WorldsManager {
    worlds: HashMap<String, WorldManager>,
}

impl WorldsManager {
    pub fn new() -> Self {
        WorldsManager {
            worlds: HashMap::new(),
        }
    }
}
