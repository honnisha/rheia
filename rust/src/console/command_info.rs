use clap::{Command, Arg};
use serde::Deserialize;


#[derive(Debug, Clone, Deserialize)]
struct ArgInfo {
    name: String,

    short: Option<char>,
    long: Option<String>,
    help: Option<String>,
    conflicts_with: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommandInfo {
    pub name: String,

    about: Option<String>,
    version: Option<String>,
    subcommand_required: Option<bool>,
    arg_required_else_help: Option<bool>,

    short_flag: Option<char>,
    long_flag: Option<String>,

    args: Option<Vec<ArgInfo>>,
    subcommands: Option<Vec<CommandInfo>>,
}

impl CommandInfo {
    pub fn eval(&self) -> Command {
        CommandInfo::get_command(self)
    }

    fn get_command(c: &CommandInfo) -> Command {
        let mut command = Command::new(c.name.clone());
        if let Some(v) = c.version.clone() {
            command = command.version(v);
        }
        if let Some(v) = c.subcommand_required.clone() {
            command = command.subcommand_required(v);
        }
        if let Some(v) = c.arg_required_else_help.clone() {
            command = command.arg_required_else_help(v);
        }
        if let Some(v) = c.about.clone() {
            command = command.about(v);
        }
        if let Some(v) = c.short_flag.clone() {
            command = command.short_flag(v);
        }
        if let Some(v) = c.long_flag.clone() {
            command = command.long_flag(v);
        }

        if c.subcommands.is_some() {
            for subcommand_info in c.subcommands.as_ref().unwrap().clone() {
                command = command.subcommand(CommandInfo::get_command(&subcommand_info));
            }
        }
        if c.args.is_some() {
            for arg_info in c.args.as_ref().unwrap().clone() {
                command = command.arg(CommandInfo::get_arg(&arg_info));
            }
        }
        command
    }

    fn get_arg(a: &ArgInfo) -> Arg {
        let mut arg = Arg::new(a.name.clone());
        if let Some(v) = a.short.clone() {
            arg = arg.short(v);
        }
        if let Some(v) = a.long.clone() {
            arg = arg.long(v);
        }
        if let Some(v) = a.help.clone() {
            arg = arg.help(v);
        }
        if let Some(v) = a.conflicts_with.clone() {
            arg = arg.conflicts_with(v);
        }
        arg
    }
}
