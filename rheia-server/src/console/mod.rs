use bevy_app::{App, Plugin, Update, Startup};
use bevy_ecs::{system::ResMut, world::World};

use crate::network::runtime_plugin::RuntimePlugin;

use self::{console_handler::ConsoleHandler, commands_executer::CommandsHandler, console_sender::Console, completer::{CustomCompleter, CompleteResponse}};

pub mod commands_executer;
pub mod console_handler;
pub mod console_sender;
pub mod helper;
pub mod completer;
pub mod command;

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
        let mut response = CompleteResponse::new(request);
        let sender = Console::default();
        CommandsHandler::complete(world, Box::new(sender), &mut response);
        CustomCompleter::send_complete_response(response);
    }
}

fn run_handler(mut console_handler: ResMut<ConsoleHandler>) {
    if RuntimePlugin::is_stopped() {
        return;
    }
    console_handler.run_handler();
}
