use args::MainCommand;
use bevy::time::TimePlugin;
use bevy_app::{App, ScheduleRunnerPlugin};
use bevy_ecs::system::Resource;
use clap::Parser;

use client_resources::ResourcesPlugin;
use network::NetworkPlugin;

use crate::console::{console_handler::ConsoleHandler, ConsolePlugin};

mod args;
mod client_resources;
mod console;
mod network;
mod worlds;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Resource)]
pub struct ServerSettings {
    args: MainCommand,
}

impl ServerSettings {
    pub fn get_args(&self) -> &MainCommand {
        &self.args
    }
}

pub fn console_send(message: String) {
    ConsoleHandler::send_message(message);
}

fn main() {
    let server_settings = ServerSettings {
        args: MainCommand::parse(),
    };

    console_send(format!("HonnyCraft Server version {}", VERSION));

    App::new()
        .add_plugin(TimePlugin::default())
        .add_plugin(ScheduleRunnerPlugin::default())
        .insert_resource(server_settings)
        .add_plugin(NetworkPlugin::default())
        .add_plugin(ResourcesPlugin::default())
        .add_plugin(ConsolePlugin::default())
        .run();
}
