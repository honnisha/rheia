use std::sync::Arc;

use ahash::HashMap;
use bevy::prelude::Resource;
use bevy::time::Time;
use bevy_ecs::system::Res;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use super::world_manager::WorldManager;

type WorldsType = HashMap<String, Arc<RwLock<WorldManager>>>;

/// Contains and manages all worlds of the server
#[derive(Resource)]
pub struct WorldsManager {
    worlds: WorldsType,
}

impl Default for WorldsManager {
    fn default() -> Self {
        WorldsManager {
            worlds: Default::default(),
        }
    }
}

impl WorldsManager {
    pub fn has_world_with_slug(&self, slug: &String) -> bool {
        self.worlds.contains_key(slug)
    }

    pub fn create_world(&mut self, slug: String, seed: u64) -> Result<(), String> {
        if self.worlds.contains_key(&slug) {
            return Err(format!("World with slug \"{}\" already exists", slug));
        }
        self.worlds
            .insert(slug.clone(), Arc::new(RwLock::new(WorldManager::new(slug, seed))));
        Ok(())
    }

    pub fn count(&self) -> usize {
        self.worlds.len()
    }

    pub fn get_worlds(&self) -> &WorldsType {
        &self.worlds
    }

    pub fn get_world_manager(&self, key: &String) -> Option<RwLockReadGuard<WorldManager>> {
        match self.worlds.get(key) {
            Some(w) => Some(w.read()),
            None => None,
        }
    }

    pub fn get_world_manager_mut(&self, key: &String) -> Option<RwLockWriteGuard<WorldManager>> {
        match self.worlds.get(key) {
            Some(w) => Some(w.write()),
            None => return None,
        }
    }
}

pub fn update_world_chunks(worlds_manager: Res<WorldsManager>, time: Res<Time>) {
    for (_key, world) in worlds_manager.get_worlds().iter() {
        world.write().update_chunks(time.delta());
    }
}
