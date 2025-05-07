use bevy_app::{App, Plugin, Startup, Update};
pub mod bevy_commands;
pub mod load_worlds;

use crate::console::commands_executer::{CommandExecuter, CommandsHandler};

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

        let worlds_manager = WorldsManager::default();
        app.insert_resource(worlds_manager);

        app.add_systems(Startup, load_worlds::load_worlds);
        app.add_systems(Update, update_world_chunks);
        app.add_systems(Update, on_chunk_loaded::on_chunk_loaded);
    }
}
