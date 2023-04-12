use clap::{Command};
use serde::Deserialize;


#[derive(Debug, Clone, Deserialize)]
struct ArgInfo {
    name: String,

    short: Option<String>,
    long: Option<String>,
    help: Option<String>,
    conflicts_with: Option<String>,
    action: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct SubcommandInfo {
    name: String,

    short_flag: Option<String>,
    long_flag: Option<String>,
    about: Option<String>,
    args: Vec<ArgInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommandInfo {
    pub name: String,

    about: Option<String>,
    version: Option<String>,
    subcommand_required: Option<String>,
    arg_required_else_help: Option<String>,

    subcommands: Vec<SubcommandInfo>,
}

impl CommandInfo {
    pub fn get_command(&self) -> Command {
        let command = Command::new(self.name.clone());
        command
    }
}
