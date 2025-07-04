use bevy_app::{App, Plugin, Startup, Update};
use bevy_ecs::{system::ResMut, world::World};
use completer::CustomCompleter;

use crate::network::runtime_plugin::RuntimePlugin;

use self::{commands_executer::CommandsHandler, console_handler::ConsoleHandler, console_sender::Console};

pub mod commands_executer;
pub mod completer;
pub mod console_handler;
pub mod console_sender;
pub mod helper;

pub struct ConsolePlugin;

impl Default for ConsolePlugin {
    fn default() -> Self {
        Self {}
    }
}

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CommandsHandler::default());
        app.insert_resource(ConsoleHandler::default());
        app.add_systems(Update, handler_console_input);
        app.add_systems(Update, handler_console_complete);
        app.add_systems(Startup, run_handler);
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
        let sender = Console::default();
        let complete_response = CommandsHandler::complete(world, Box::new(sender), &request);
        if complete_response.get_completions().len() > 0 {
            CustomCompleter::send_complete_response(complete_response);
        }
    }
}

fn run_handler(mut console_handler: ResMut<ConsoleHandler>) {
    if RuntimePlugin::is_stopped() {
        return;
    }
    console_handler.run_handler();
}
