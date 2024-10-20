use bevy::prelude::Resource;
use clap::Parser;
use std::env;
use std::path::PathBuf;

use crate::args::MainCommand;

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
}
