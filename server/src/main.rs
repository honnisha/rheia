use args::MainCommand;
use bevy::time::TimePlugin;
use bevy_app::App;
use bevy_ecs::system::Resource;
use clap::Parser;

use client_resources::ResourcesPlugin;
use crate::network::NetworkPlugin;
use worlds::WorldsHandlerPlugin;

use crate::console::{console_handler::ConsoleHandler, ConsolePlugin};

mod args;
mod client_resources;
mod console;
mod worlds;
mod network;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Resource, Clone, Debug)]
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

    let mut app = App::new();
    app.add_plugin(TimePlugin::default());
    app.insert_resource(server_settings);
    app.add_plugin(ResourcesPlugin::default());
    app.add_plugin(ConsolePlugin::default());
    app.add_plugin(WorldsHandlerPlugin::default());

    NetworkPlugin::build(&mut app);
    app.run();
}
