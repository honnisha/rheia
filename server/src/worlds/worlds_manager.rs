use bevy::prelude::Resource;
use dashmap::DashMap;

use super::world_manager::WorldManager;

#[derive(Resource)]
pub struct WorldsManager {
    worlds: DashMap<String, WorldManager>,
}

impl WorldsManager {
    pub fn new() -> Self {
        WorldsManager {
            worlds: DashMap::new(),
        }
    }
}
