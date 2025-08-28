use bevy_ecs::system::{Res, ResMut};
use bracket_lib::random::RandomNumberGenerator;
use common::world_generator::default::WorldGeneratorSettings;

use crate::{
    client_resources::server_settings::ServerSettings, launch_settings::LaunchSettings,
    network::runtime_plugin::RuntimePlugin,
};

use super::worlds_manager::WorldsManager;

pub(crate) fn load_worlds(
    launch_settings: Res<LaunchSettings>,
    mut worlds_manager: ResMut<WorldsManager>,
    server_settings: Res<ServerSettings>,
) {
    if RuntimePlugin::is_stopped() {
        return;
    }

    let world_storage_settings = launch_settings.get_world_storage_settings();

    if let Err(e) = worlds_manager.scan_worlds(&world_storage_settings, server_settings.get_block_id_map()) {
        log::error!(target: "worlds", "&cWorlds loading error!");
        log::error!(target: "worlds", "{}", e);
        RuntimePlugin::stop();
        return;
    }

    let default_world = "default".to_string();
    if worlds_manager.count() == 0 && !worlds_manager.has_world_with_slug(&default_world) {
        let mut rng = RandomNumberGenerator::new();
        let seed = rng.next_u64();

        let world = worlds_manager.create_world(
            default_world.clone(),
            seed,
            WorldGeneratorSettings::default(),
            &world_storage_settings,
            server_settings.get_block_id_map(),
        );
        match world {
            Ok(_) => {
                log::info!(target: "worlds", "&dDefault world &5\"{}\"&d was created", default_world);
            }
            Err(e) => {
                log::error!(target: "worlds", "Error with creating &e\"{}\"&r world", default_world);
                log::error!(target: "worlds", "Error: {}", e);
                RuntimePlugin::stop();
                return;
            }
        }
    }
}
