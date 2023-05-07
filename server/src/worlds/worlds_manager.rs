use std::collections::HashMap;

use bevy::prelude::Resource;

use super::world_manager::WorldManager;

#[derive(Resource)]
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
