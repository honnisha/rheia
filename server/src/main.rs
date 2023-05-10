use args::MainCommand;
use bevy::{
    prelude::{FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin},
    time::TimePlugin,
};
use bevy_app::{App, ScheduleRunnerPlugin};
use bevy_ecs::system::Resource;
use clap::Parser;
use log::{info, LevelFilter};

use crate::network::NetworkPlugin;
use crate::{logger::CONSOLE_LOGGER, network::runtime::RuntimePlugin};
use client_resources::ResourcesPlugin;
use worlds::WorldsHandlerPlugin;

use crate::console::ConsolePlugin;

mod args;
mod client_resources;
mod console;
mod logger;
mod network;
mod worlds;

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

fn main() {
    log::set_logger(&CONSOLE_LOGGER).unwrap();
    log::set_max_level(LevelFilter::Info);

    let server_settings = ServerSettings {
        args: MainCommand::parse(),
    };

    info!("HonnyCraft Server version {}", VERSION);

    let mut app = App::new();
    app.add_plugin(TimePlugin::default());
    app.add_plugin(TaskPoolPlugin::default());
    app.add_plugin(TypeRegistrationPlugin::default());
    app.add_plugin(FrameCountPlugin::default());
    app.add_plugin(ScheduleRunnerPlugin::default());

    app.insert_resource(server_settings);
    app.add_plugin(RuntimePlugin::default());
    app.add_plugin(ResourcesPlugin::default());
    app.add_plugin(ConsolePlugin::default());
    app.add_plugin(WorldsHandlerPlugin::default());

    NetworkPlugin::build(&mut app);
    app.run();
}
