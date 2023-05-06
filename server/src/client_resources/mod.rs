use bevy_app::{Plugin, App};

use self::resources_manager::ResourceManager;

pub mod resources_manager;
pub mod resource_instance;


pub struct ResourcesPlugin;

impl Default for ResourcesPlugin {
    fn default() -> Self {
        Self {}
    }
}

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        let mut resource_manager = ResourceManager::new();
        resource_manager.rescan_scripts();
        app.insert_resource(resource_manager);
    }
}
