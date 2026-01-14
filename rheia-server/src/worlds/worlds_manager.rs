use std::{collections::BTreeMap, sync::Arc};

use ahash::HashMap;
use bevy::prelude::Resource;
use bevy::time::Time;
use bevy_ecs::system::Res;
use common::{
    WorldStorageManager,
    chunks::chunk_data::BlockIndexType,
    world_generator::default::WorldGeneratorSettings,
    worlds_storage::taits::{IWorldStorage, WorldStorageSettings},
};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use super::world_manager::WorldManager;

type WorldsType = HashMap<String, Arc<RwLock<WorldManager>>>;

/// Contains and managers of all worlds of the server
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
    pub fn scan_worlds(
        &mut self,
        world_storage_settings: &WorldStorageSettings,
        block_id_map: &BTreeMap<BlockIndexType, String>,
    ) -> Result<(), String> {
        let worlds_info = match WorldStorageManager::scan_worlds(world_storage_settings) {
            Ok(w) => w,
            Err(e) => {
                return Err(e.to_string());
            }
        };
        for world_info in worlds_info {
            if let Err(e) = self.create_world(
                world_info.slug.clone(),
                world_info.seed,
                WorldGeneratorSettings::default(),
                &world_storage_settings,
                block_id_map,
            ) {
                return Err(e.to_string());
            };
            log::info!(target: "worlds", "World &a\"{}\"&r loaded", world_info.slug);
        }
        Ok(())
    }

    pub fn has_world_with_slug(&self, slug: &String) -> bool {
        self.worlds.contains_key(slug)
    }

    pub fn save_all(&self) -> Result<(), String> {
        for (_world_slug, world) in self.worlds.iter() {
            world.write().save()?;
        }
        Ok(())
    }

    pub fn create_world(
        &mut self,
        slug: String,
        seed: u64,
        world_settings: WorldGeneratorSettings,
        world_storage_settings: &WorldStorageSettings,
        block_id_map: &BTreeMap<BlockIndexType, String>,
    ) -> Result<(), String> {
        if self.worlds.contains_key(&slug) {
            return Err(format!("&cWorld with slug &4\"{}\"&c already exists", slug));
        }
        let world = match WorldManager::new(slug.clone(), seed, world_settings, world_storage_settings, block_id_map) {
            Ok(w) => w,
            Err(e) => return Err(format!("&cWorld &4\"{}\"&c error: {}", slug, e)),
        };
        self.worlds.insert(slug, Arc::new(RwLock::new(world)));
        Ok(())
    }

    pub fn count(&self) -> usize {
        self.worlds.len()
    }

    pub fn get_worlds(&self) -> &WorldsType {
        &self.worlds
    }

    pub fn get_world_manager(&self, key: &String) -> Option<RwLockReadGuard<'_, WorldManager>> {
        match self.worlds.get(key) {
            Some(w) => Some(w.read()),
            None => None,
        }
    }

    pub fn get_world_manager_mut(&self, key: &String) -> Option<RwLockWriteGuard<'_, WorldManager>> {
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
