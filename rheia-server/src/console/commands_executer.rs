use std::{error::Error, fmt};

use super::{
    command::{Command, CommandMatch},
    completer::CompleteResponse,
    console_sender::ConsoleSenderType,
};
use bevy_ecs::{system::Resource, world::World};
use log::error;
use regex::Regex;

pub const REGEX_COMMAND: &str = r####"([\d\w$&+,:;=?@#|'<>.^*()%!-]*)|"([\d\w$&+,:;=?@#|'<>.^*()%!\- ]*)""####;

// https://github.com/clap-rs/clap/blob/master/examples/pacman.rs

#[derive(Debug, Clone)]
pub struct CommandError(pub String);

impl Error for CommandError {}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

type CommandFN = fn(world: &mut World, sender: Box<dyn ConsoleSenderType>, CommandMatch) -> Result<(), CommandError>;

#[derive(Clone)]
pub struct CommandExecuter {
    command_parser: Command,
    handler: CommandFN,
    name: String,
}

impl CommandExecuter {
    pub fn new(command_parser: Command, handler: CommandFN) -> Self {
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

    pub fn parse_command(command: &String) -> Vec<String> {
        let re = Regex::new(REGEX_COMMAND).unwrap();
        re.find_iter(&command).map(|e| e.as_str().to_string()).collect()
    }

    pub fn execute_command(world: &mut World, sender: Box<dyn ConsoleSenderType>, command: &String) {
        let command_sequence = CommandsHandler::parse_command(command);
        if command_sequence.len() == 0 {
            return;
        }
        let lead_command = command_sequence[0].clone();

        let mut handlers = world.resource_mut::<CommandsHandler>();
        let mut handler: Option<(CommandFN, CommandMatch)> = None;

        for command_handler in handlers.commands.iter_mut() {
            if command_handler.name != lead_command {
                continue;
            }

            let command = command_handler.command_parser.clone();
            match command.eval(&command_sequence[1..]) {
                Ok(e) => {
                    handler = Some((command_handler.handler.clone(), e.clone()));
                }
                Err(e) => {
                    sender.send_console_message(e.to_string());
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

    pub fn complete(world: &mut World, _sender: Box<dyn ConsoleSenderType>, complete_response: &mut CompleteResponse) {
        let handlers = world.resource::<CommandsHandler>();

        let line = complete_response.get_request().get_line().clone();
        let pos = complete_response.get_request().get_pos().clone();

        let command_sequence = CommandsHandler::parse_command(&line[..pos].to_string());

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

        for command_handler in handlers.commands.iter() {
            if command_handler.name != lead_command {
                continue;
            }

            let command = command_handler.command_parser.clone();
            let last_arg = command_sequence[command_sequence.len() - 1].clone();

            if let Some((command, arg)) = command.get_current(&command_sequence[1..]) {
                match arg {
                    Some(_a) => {},
                    None => {
                        // println!("command:{} arg:{:?} &command_sequence[1..]:{:?}", command.get_name(), arg, &command_sequence[1..]);
                        for c in command.commands() {

                            // if command name starts with arg name
                            if c.get_name().starts_with(&last_arg) {
                                let complete = c.get_name()[last_arg.len()..].to_string();
                                complete_response.add_completion(complete);
                            }
                        }
                    },
                }
            }
            break;
        }

        //if let Some(h) = complete_handler {
        //    (h)(world, sender, complete_response);
        //}
    }
}
