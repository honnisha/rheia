use super::{completer::CompleteResponse, console_sender::ConsoleSenderType};
use bevy_ecs::{system::Resource, world::World};
use clap::{error::ErrorKind, ArgMatches, Command};
use log::error;
use regex::Regex;

pub const REGEX_COMMAND: &str = r####"([\d\w$&+,:;=?@#|'<>.^*()%!-]+)|"([\d\w$&+,:;=?@#|'<>.^*()%!\- ]+)""####;

// https://github.com/clap-rs/clap/blob/master/examples/pacman.rs
pub type CommandError = String;
type CommandFN = fn(world: &mut World, sender: Box<dyn ConsoleSenderType>, ArgMatches) -> Result<(), CommandError>;
type CommandCompleteFN = fn(
    world: &mut World,
    sender: Box<dyn ConsoleSenderType>,
    complete_response: &mut CompleteResponse,
    command_sequence: Vec<String>,
) -> Result<(), CommandError>;

#[derive(Clone)]
pub struct CommandExecuter {
    command_parser: Command,
    handler: CommandFN,
    complete_handler: Option<CommandCompleteFN>,
    name: String,
}

impl CommandExecuter {
    pub fn new(command_parser: Command, handler: CommandFN, complete_handler: Option<CommandCompleteFN>) -> Self {
        let name = command_parser.get_name().to_string();
        Self {
            command_parser,
            handler,
            complete_handler,
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

    pub fn execute_command(world: &mut World, sender: Box<dyn ConsoleSenderType>, command: &String) {
        let re = Regex::new(REGEX_COMMAND).unwrap();
        let command_sequence: Vec<String> = re.find_iter(&command).map(|e| e.as_str().to_string()).collect();
        if command_sequence.len() == 0 {
            return;
        }
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
                let sender_title = format!("{}", sender);
                if let Err(e) = (h)(world, sender, args) {
                    error!("Command {} by:{} error: {}", command, sender_title, e);
                }
            }
            None => {
                sender.send_console_message(format!("Command \"{}\" not found", command));
            }
        };
    }

    pub fn complete(world: &mut World, sender: Box<dyn ConsoleSenderType>, complete_response: &mut CompleteResponse) {
        let handlers = world.resource::<CommandsHandler>();

        let line = complete_response.get_request().get_line().clone();
        let pos = complete_response.get_request().get_pos().clone();

        let re = Regex::new(REGEX_COMMAND).unwrap();
        let command_sequence: Vec<String> = re.find_iter(&line).map(|e| e.as_str().to_string()).collect();

        // Return all command names
        if command_sequence.len() == 0 {
            for command_handler in handlers.commands.iter() {
                complete_response.add_completion(command_handler.name.clone());
            }
            return;
        }
        let lead_command = command_sequence[0].clone();

        // Complete command name
        if pos <= lead_command.len() {
            for command_handler in handlers.commands.iter() {
                if command_handler.name.starts_with(&line[..pos]) {
                    complete_response.add_completion(command_handler.name[pos..].to_string());
                }
            }
            return;
        }

        let mut complete_handler: Option<CommandCompleteFN> = None;
        for command_handler in handlers.commands.iter() {
            if command_handler.name != lead_command {
                continue;
            }
            complete_handler = command_handler.complete_handler;
            break;
        }

        if let Some(h) = complete_handler {
            (h)(world, sender, complete_response);
        }
    }
}
