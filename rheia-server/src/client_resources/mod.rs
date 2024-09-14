use bevy_app::{App, Plugin, Startup};
use resources_manager::rescan_scripts;

use self::resources_manager::ResourceManager;

pub mod resource_instance;
pub mod resources_manager;

pub struct ResourcesPlugin;

impl Default for ResourcesPlugin {
    fn default() -> Self {
        Self {}
    }
}

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        let resource_manager = ResourceManager::new();
        app.insert_resource(resource_manager);

        app.add_systems(Startup, rescan_scripts);
    }
}
