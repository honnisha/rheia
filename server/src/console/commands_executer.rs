use super::console_sender::ConsoleSenderType;
use bevy_ecs::{system::Resource, world::World};
use clap::{error::ErrorKind, Command, ArgMatches};
use regex::Regex;

pub const REGEX_COMMAND: &str = r####"([\d\w$&+,:;=?@#|'<>.^*()%!-]+)|"([\d\w$&+,:;=?@#|'<>.^*()%!\- ]+)""####;

// https://github.com/clap-rs/clap/blob/master/examples/pacman.rs
type CommandFN = fn(world: &mut World, sender: &dyn ConsoleSenderType, ArgMatches);

#[derive(Clone)]
pub struct CommandExecuter {
    command_parser: Command,
    handler: CommandFN,
    name: String,
}

impl CommandExecuter {
    pub fn new(command_parser: Command, handler: CommandFN, ) -> Self {
        let name = command_parser.get_name().to_string();
        Self {
            command_parser,
            handler,
            name,
        }
    }
}

#[derive(Resource)]
pub struct CommandsHandler {
    commands: Vec<CommandExecuter>,
}

impl Default for CommandsHandler {
    fn default() -> Self {
        Self { commands: Vec::new() }
    }
}

impl CommandsHandler {
    pub fn add_command_executer(&mut self, executer: CommandExecuter) {
        self.commands.push(executer);
    }

    pub fn execute_command(world: &mut World, sender: &dyn ConsoleSenderType, command: &String) {
        let re = Regex::new(REGEX_COMMAND).unwrap();
        let command_sequence: Vec<String> = re.find_iter(&command).map(|e| e.as_str().to_string()).collect();
        let lead_command = command_sequence[0].clone();

        let mut handlers = world.resource_mut::<CommandsHandler>();
        let mut handler: Option<(CommandFN, ArgMatches)> = None;

        for command_handler in handlers.commands.iter_mut() {
            if command_handler.name != lead_command {
                continue;
            }

            let parser = command_handler.command_parser.clone();
            match parser.try_get_matches_from(&command_sequence) {
                Ok(e) => {
                    handler = Some((command_handler.handler.clone(), e.clone()));
                }
                Err(e) => {
                    match e.kind() {
                        ErrorKind::DisplayHelp | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand => {
                            let mut buf = Vec::new();
                            command_handler.command_parser.write_help(&mut buf).unwrap();
                            sender.send_console_message(String::from_utf8(buf).unwrap())
                        }
                        _ => {
                            sender.send_console_message(e.render().to_string());
                        }
                    };
                    return;
                }
            };
        }
        match handler {
            Some((h, args)) => {
                (h)(world, sender, args);
            }
            None => {
                sender.send_console_message(format!("Command \"{}\" not found", command));
            }
        };
    }
}
