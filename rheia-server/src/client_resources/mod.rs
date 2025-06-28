use bevy_app::{App, Plugin, Startup};
use bevy_ecs::schedule::IntoScheduleConfigs;
use resources_manager::rescan_resources;
use server_settings::{rescan_server_settings, setup_default_blocks, ServerSettings};

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
        app.insert_resource(ResourceManager::default());
        app.insert_resource(ServerSettings::default());

        app.add_systems(Startup, setup_default_blocks);
        app.add_systems(Startup, rescan_resources.after(setup_default_blocks));

        app.add_systems(Startup, rescan_server_settings.after(rescan_resources));
    }
}
