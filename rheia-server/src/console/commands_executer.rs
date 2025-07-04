use super::console_sender::ConsoleSenderType;
use bevy_ecs::{resource::Resource, world::World};
use common::commands::{
    command::{Command, CommandMatch},
    complitions::{CompleteRequest, CompleteResponse},
};
use log::error;

// https://github.com/clap-rs/clap/blob/master/examples/pacman.rs

type CommandFN = fn(world: &mut World, sender: Box<dyn ConsoleSenderType>, CommandMatch) -> Result<(), String>;

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

    pub fn execute_command(world: &mut World, sender: Box<dyn ConsoleSenderType>, command: &String) {
        let command_sequence = Command::parse_command(command);
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
                Ok(command_match) => {
                    handler = Some((command_handler.handler.clone(), command_match.clone()));
                    break;
                }
                Err(e) => {
                    sender.send_console_message(e.to_string());
                    return;
                }
            };
        }
        match handler {
            Some((handler_fn, command_match)) => {
                let sender_title = format!("{}", sender);
                if let Err(e) = (handler_fn)(world, sender, command_match) {
                    error!("Command {} by:{} error: {}", command, sender_title, e);
                }
            }
            None => {
                sender.send_console_message(format!("&cCommand &4\"{}\" &cnot found", command));
            }
        };
    }

    pub fn complete(
        world: &mut World,
        _sender: Box<dyn ConsoleSenderType>,
        request: &CompleteRequest,
    ) -> CompleteResponse {
        let handlers = world.resource::<CommandsHandler>();

        let commands: Vec<Command> = handlers.commands.iter().map(|m| m.command_parser.clone()).collect();
        let complete_response = CompleteResponse::complete(request, commands.iter());
        complete_response

        //if let Some(h) = complete_handler {
        //    (h)(world, sender, complete_response);
        //}
    }
}
