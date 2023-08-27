use bevy_app::{App, Plugin, Update};
use bracket_lib::random::RandomNumberGenerator;
use log::info;

use crate::console::commands_executer::{CommandExecuter, CommandsHandler};

use self::{
    commands::{get_command_parser, world_command},
    worlds_manager::{chunk_loaded_event_reader, update_world_chunks, WorldsManager},
};

pub mod chunks;
pub mod commands;
pub mod world_generator;
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
        let mut commands_handler = app.world.get_resource_mut::<CommandsHandler>().unwrap();
        commands_handler.add_command_executer(CommandExecuter::new(get_command_parser(), world_command));

        let mut wm = WorldsManager::default();

        let default_world = "default".to_string();
        if wm.count() == 0 && !wm.has_world_with_slug(&default_world) {
            let mut rng = RandomNumberGenerator::new();
            let seed = rng.next_u64();

            match wm.create_world(default_world.clone(), seed) {
                Ok(_) => {
                    info!("Default world \"{}\" was created", default_world);
                }
                Err(e) => {
                    info!("Error with creating \"{}\" world: {}", default_world, e);
                }
            }
        }
        app.insert_resource(wm);

        app.add_systems(Update, update_world_chunks);
        app.add_systems(Update, chunk_loaded_event_reader);
    }
}
