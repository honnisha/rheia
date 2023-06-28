use bevy_app::{App, Plugin};

use self::{console_handler::ConsoleHandler, commands_executer::CommandsHandler};

pub mod commands_executer;
pub mod console_handler;
pub mod console_sender;

pub struct ConsolePlugin;

impl Default for ConsolePlugin {
    fn default() -> Self {
        Self {}
    }
}

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CommandsHandler::default());
        app.add_system(ConsoleHandler::handler_console_input);
        app.add_startup_system(ConsoleHandler::run_handler);
    }
}
