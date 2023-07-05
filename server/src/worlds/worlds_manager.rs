use bevy::prelude::Resource;
use dashmap::DashMap;

use super::world_manager::WorldManager;

#[derive(Resource)]
pub struct WorldsManager {
    worlds: DashMap<String, WorldManager>,
}

impl Default for WorldsManager {
    fn default() -> Self {
        WorldsManager {
            worlds: DashMap::new(),
        }
    }
}

impl WorldsManager {
    pub fn has_world_with_slug(&self, slug: &String) -> bool {
        self.worlds.contains_key(slug)
    }

    pub fn create_world(&mut self, slug: String) -> Result<(), String> {
        if self.worlds.contains_key(&slug) {
            return Err(format!("World with slug \"{}\" already exists", slug));
        }
        self.worlds.insert(slug.clone(), WorldManager::new(slug));
        Ok(())
    }

    pub fn count(&self) -> usize {
        self.worlds.len()
    }

    pub fn get_worlds(&self) -> &DashMap<String, WorldManager> {
        &self.worlds
    }

    pub fn get_world(&self, key: &String) -> Option<dashmap::mapref::one::Ref<'_, String, WorldManager>> {
        self.worlds.get(key)
    }

    pub fn get_world_mut(&self, key: &String) -> Option<dashmap::mapref::one::RefMut<'_, String, WorldManager>> {
        self.worlds.get_mut(key)
    }
}
