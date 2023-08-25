use bevy::prelude::*;
use bevy_app::App;

use super::{
    chunks::chunk_generator::{chunk_generator, GenerateChunkEvent},
    world_manager::{chunks_loader_system, WorldManager},
};

struct WorldManagerPlugin;

impl Plugin for WorldManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, chunks_loader_system);
        app.insert_resource(WorldsManager::default());

        app.add_event::<GenerateChunkEvent>();
        app.add_systems(Update, chunk_generator);
    }
}

#[derive(Resource, Default)]
pub struct WorldsManager {
    world: Option<WorldManager>,
}

impl WorldsManager {
    pub fn get_world_slug(&self) -> Option<&String> {
        match self.world.as_ref() {
            Some(w) => Some(w.get_slug()),
            None => None,
        }
    }

    pub fn get_world(&mut self) -> Option<&WorldManager> {
        match self.world.as_ref() {
            Some(w) => Some(w),
            None => None,
        }
    }

    pub fn get_world_mut(&mut self) -> Option<&mut WorldManager> {
        match self.world.as_mut() {
            Some(w) => Some(w),
            None => None,
        }
    }

    pub fn load_world(&mut self, slug: String) {
        let world = WorldManager::new(slug);
        self.world = Some(world);
    }

    pub fn unload_world(&mut self) {
        self.world = None;
    }
}
