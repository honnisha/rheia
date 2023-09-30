use bevy_app::{App, Plugin, Update, Startup};
use bevy_ecs::world::World;

use self::{console_handler::ConsoleHandler, commands_executer::CommandsHandler, console_sender::Console, completer::CustomCompleter};

pub mod commands_executer;
pub mod console_handler;
pub mod console_sender;
pub mod helper;
pub mod completer;

pub struct ConsolePlugin;

impl Default for ConsolePlugin {
    fn default() -> Self {
        Self {}
    }
}

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CommandsHandler::default());
        app.add_systems(Update, handler_console_input);
        app.add_systems(Update, handler_console_complete);
        app.add_systems(Startup, ConsoleHandler::run_handler);
    }
}

fn handler_console_input(world: &mut World) {
    for command in ConsoleHandler::iter_commands() {
        let sender = Console::default();
        CommandsHandler::execute_command(world, Box::new(sender), &command);
    }
}

fn handler_console_complete(world: &mut World) {
    for request in CustomCompleter::iter_complere_requests() {
        log::info!("pos:{} line:{}", request.pos, request.line);
    }
}
