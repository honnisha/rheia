use bevy_app::{App, Plugin, Startup};
use resources_manager::rescan_scripts;
use server_settings::{rescan_server_settings, ServerSettings};

use self::resources_manager::ResourceManager;

pub mod resource_instance;
pub mod resources_manager;
pub mod server_settings;

pub struct ResourcesPlugin;

impl Default for ResourcesPlugin {
    fn default() -> Self {
        Self {}
    }
}

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ResourceManager::new());
        app.add_systems(Startup, rescan_scripts);

        app.insert_resource(ServerSettings::new());
        app.add_systems(Startup, rescan_server_settings);
    }
}
