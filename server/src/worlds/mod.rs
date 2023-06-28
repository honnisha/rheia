use bevy_app::{App, Plugin};

use crate::console::commands_executer::{CommandsHandler, CommandExecuter};

use self::{worlds_manager::WorldsManager, commands::{world_command, get_command_parser}};

pub mod world_manager;
pub mod worlds_manager;
pub mod commands;

pub struct WorldsHandlerPlugin;

impl Default for WorldsHandlerPlugin {
    fn default() -> Self {
        Self {}
    }
}

impl Plugin for WorldsHandlerPlugin {
    fn build(&self, app: &mut App) {
        let mut commands_handler = app.world.get_resource_mut::<CommandsHandler>().unwrap();
        commands_handler.add_command_executer(CommandExecuter::new(
            get_command_parser(),
            world_command,
        ));

        app.insert_resource(WorldsManager::default());
    }
}
