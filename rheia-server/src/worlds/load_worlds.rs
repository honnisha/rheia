use bevy_ecs::system::{Res, ResMut};
use bracket_lib::random::RandomNumberGenerator;
use common::world_generator::default::WorldGeneratorSettings;

use crate::launch_settings::LaunchSettings;

use super::worlds_manager::WorldsManager;

pub(crate) fn load_worlds(launch_settings: Res<LaunchSettings>, mut worlds_manager: ResMut<WorldsManager>) {
    let world_storage_settings = launch_settings.get_world_storage_settings();

    worlds_manager.scan_worlds(&world_storage_settings);

    let default_world = "default".to_string();
    if worlds_manager.count() == 0 && !worlds_manager.has_world_with_slug(&default_world) {
        let mut rng = RandomNumberGenerator::new();
        let seed = rng.next_u64();

        let world = worlds_manager.create_world(
            default_world.clone(),
            seed,
            WorldGeneratorSettings::default(),
            &world_storage_settings,
        );
        match world {
            Ok(_) => {
                log::info!(target: "worlds", "Default world &a\"{}\"&r was created", default_world);
            }
            Err(e) => {
                log::error!(target: "worlds", "Error with creating &e\"{}\"&r world", default_world);
                log::error!(target: "worlds", "Error: {}", e);
                panic!();
            }
        }
    }
}
