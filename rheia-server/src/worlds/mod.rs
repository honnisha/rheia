use bevy_app::{App, Plugin, Update};
use bracket_lib::random::RandomNumberGenerator;
use common::world_generator::default::WorldGeneratorSettings;
use log::info;

pub mod bevy_commands;

use crate::{
    console::commands_executer::{CommandExecuter, CommandsHandler},
    launch_settings::LaunchSettings,
};

use self::{
    commands::{command_parser_teleport, command_parser_world, command_teleport, command_world},
    worlds_manager::{update_world_chunks, WorldsManager},
};

pub mod chunks;
pub mod commands;
pub mod ecs;
pub mod on_chunk_loaded;
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
        let mut commands_handler = app.world_mut().get_resource_mut::<CommandsHandler>().unwrap();
        commands_handler.add_command_executer(CommandExecuter::new(command_parser_world(), command_world));
        commands_handler.add_command_executer(CommandExecuter::new(command_parser_teleport(), command_teleport));

        let mut worlds_manager = WorldsManager::default();

        let launch_settings = app.world().get_resource::<LaunchSettings>().unwrap();
        let world_storage_settings = launch_settings.get_world_storage_settings();

        let default_world = "default".to_string();
        if worlds_manager.count() == 0 && !worlds_manager.has_world_with_slug(&default_world) {
            let mut rng = RandomNumberGenerator::new();
            let seed = rng.next_u64();

            let world = worlds_manager.create_world(
                default_world.clone(),
                seed,
                WorldGeneratorSettings::default(),
                world_storage_settings,
            );
            match world {
                Ok(_) => {
                    info!(target: "worlds", "Default world \"{}\" was created", default_world);
                }
                Err(e) => {
                    info!(target: "worlds", "Error with creating \"{}\" world: {}", default_world, e);
                }
            }
        }
        app.insert_resource(worlds_manager);

        app.add_systems(Update, update_world_chunks);
        app.add_systems(Update, on_chunk_loaded::on_chunk_loaded);
    }
}
