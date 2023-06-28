use std::{thread, time::Duration};

use bevy_ecs::world::World;
use chrono::Local;
use flume::{Receiver, Sender};
use lazy_static::lazy_static;
use rustyline::{error::ReadlineError, history::FileHistory, Config, DefaultEditor, ExternalPrinter};

use crate::network::runtime_plugin::RuntimePlugin;

use super::{console_sender::Console, commands_executer::CommandsHandler};

lazy_static! {
    // To handle output log from entire server and draw it on console
    static ref CONSOLE_OUTPUT_CHANNEL: (Sender<String>, Receiver<String>) = flume::unbounded();

    // Console input
    static ref CONSOLE_INPUT_CHANNEL: (Sender<String>, Receiver<String>) = flume::unbounded();
}

pub struct ConsoleHandler;

/// Read and write console std
impl ConsoleHandler {
    pub fn run_handler() {
        let config = Config::builder()
            .history_ignore_space(true)
            .auto_add_history(true)
            .edit_mode(rustyline::EditMode::Emacs)
            .build();
        let history = FileHistory::with_config(config);

        let mut rl = DefaultEditor::with_history(config, history).unwrap();
        let mut printer = rl.create_external_printer().unwrap();

        thread::spawn(move || loop {
            let readline = rl.readline("");
            match readline {
                Ok(input) => {
                    if input.len() > 0 {
                        CONSOLE_INPUT_CHANNEL.0.send(input.clone()).unwrap();
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    RuntimePlugin::stop();
                    break;
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        });

        thread::spawn(move || loop {
            ConsoleHandler::update(&mut printer);
            thread::sleep(Duration::from_millis(50));
        });
    }

    pub fn send_message(message: String) {
        let date = Local::now();
        let m = format!("{}: {}", date.format("%Y-%m-%d %H:%M:%S"), message);

        if RuntimePlugin::is_active() {
            CONSOLE_OUTPUT_CHANNEL.0.send(m).unwrap();
        } else {
            println!("{}", m);
        }
    }

    pub fn update(printer: &mut dyn ExternalPrinter) {
        for message in CONSOLE_OUTPUT_CHANNEL.1.try_iter() {
            printer.print(message).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    }

    pub fn handler_console_input(world: &mut World) {
        for command in CONSOLE_INPUT_CHANNEL.1.try_iter() {
            let sender = Console::default();
            CommandsHandler::execute_command(world, &sender, &command);
        }
    }
}
