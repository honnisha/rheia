use std::time::Duration;

use args::MainCommand;
use bevy::{
    prelude::{FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin},
    time::TimePlugin,
};
use bevy_app::{App, ScheduleRunnerPlugin};
use bevy_ecs::system::Resource;

use clap::Parser;
use log::info;

use crate::{
    args::get_log_level,
    logger::CONSOLE_LOGGER,
    network::{runtime_plugin::RuntimePlugin, server::NetworkPlugin},
};
use client_resources::ResourcesPlugin;
use worlds::WorldsHandlerPlugin;

use crate::console::ConsolePlugin;

mod args;
mod client_resources;
mod console;
mod entities;
mod events;
mod logger;
mod network;
mod worlds;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const CHUNKS_DISTANCE: u16 = 12;
pub const CHUNKS_DESPAWN_TIMER: Duration = Duration::from_secs(5);

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

    let server_settings = ServerSettings {
        args: MainCommand::parse(),
    };
    let log_level = get_log_level(&server_settings.args.logs);
    log::set_max_level(log_level.clone());
    info!("Log level using: {}", log_level);

    info!("HonnyCraft Server version {}", VERSION);

    let mut app = App::new();
    app.add_plugin(TimePlugin::default());
    app.add_plugin(TaskPoolPlugin::default());
    app.add_plugin(TypeRegistrationPlugin::default());
    app.add_plugin(FrameCountPlugin::default());
    app.add_plugin(ScheduleRunnerPlugin::default());

    app.insert_resource(server_settings);
    app.add_plugin(ConsolePlugin::default());
    app.add_plugin(RuntimePlugin::default());
    NetworkPlugin::build(&mut app);
    app.add_plugin(ResourcesPlugin::default());
    app.add_plugin(WorldsHandlerPlugin::default());
    app.run();
}
