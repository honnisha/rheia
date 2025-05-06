use bevy::prelude::Resource;
use clap::Parser;
use common::worlds_storage::taits::WorldStorageSettings;
use std::env;
use std::path::PathBuf;

use log::LevelFilter;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct MainCommand {
    #[arg(short, long, default_value_t = String::from("0.0.0.0"))]
    pub ip: String,

    #[arg(short, long, default_value_t = String::from("19132"))]
    pub port: String,

    #[arg(long, default_value_t = String::from("info"))]
    pub logs: String,

    #[arg(long = "resources-path", short = 'r')]
    pub resources_path: Option<String>,

    #[arg(long = "server-data-path", short = 'd')]
    pub server_data_path: Option<String>,
}

pub(crate) fn get_log_level(level: &String) -> LevelFilter {
    match level.to_lowercase().as_str() {
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "error" => LevelFilter::Error,
        "off" => LevelFilter::Off,
        "trace" => LevelFilter::Trace,
        "warn" => LevelFilter::Warn,
        _ => {
            panic!("Log level \"{}\" not found", level);
        }
    }
}

#[derive(Resource, Clone, Debug)]
pub struct LaunchSettings {
    args: MainCommand,
}

impl LaunchSettings {
    pub fn new() -> Self {
        Self {
            args: MainCommand::parse(),
        }
    }

    pub fn get_args(&self) -> &MainCommand {
        &self.args
    }

    pub fn get_resources_path(&self) -> PathBuf {
        match self.args.resources_path.as_ref() {
            Some(p) => PathBuf::from(shellexpand::tilde(p).to_string()),
            None => {
                let mut path = env::current_dir().unwrap().clone();
                path.push("resources");
                path
            }
        }
    }

    pub fn get_server_data_path(&self) -> PathBuf {
        match self.args.server_data_path.as_ref() {
            Some(p) => PathBuf::from(shellexpand::tilde(p).to_string()),
            None => env::current_dir().unwrap().clone(),
        }
    }

    pub fn get_world_storage_settings(&self) -> WorldStorageSettings {
        WorldStorageSettings::create(self.get_server_data_path())
    }
}
