use bevy_app::{App, Plugin};

use self::worlds_manager::WorldsManager;

pub mod entities;
pub mod world_manager;
pub mod worlds_manager;

pub struct WorldsHandlerPlugin;

impl Default for WorldsHandlerPlugin {
    fn default() -> Self {
        Self {}
    }
}

impl Plugin for WorldsHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldsManager::new());
    }
}
