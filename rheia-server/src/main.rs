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

pub const CHUNKS_DISTANCE: u16 = 16;
pub const CHUNKS_DESPAWN_TIMER: Duration = Duration::from_secs(5);

// Pack network chunk message with pallete
pub const CHUNKS_ZIP_PALLETE: bool = true;

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

    info!("Rheia Server version {}", VERSION);

    let mut app = App::new();
    app.insert_resource(server_settings);
    app.add_plugins((
        TimePlugin::default(),
        TaskPoolPlugin::default(),
        TypeRegistrationPlugin::default(),
        FrameCountPlugin::default(),
        ScheduleRunnerPlugin::default(),
        ConsolePlugin::default(),
        RuntimePlugin::default(),
        ResourcesPlugin::default(),
        WorldsHandlerPlugin::default(),
    ));
    NetworkPlugin::build(&mut app);
    app.run();
}
