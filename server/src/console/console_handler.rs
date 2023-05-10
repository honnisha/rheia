use std::{thread, time::Duration};

use bevy::prelude::Resource;
use chrono::Local;
use flume::{Receiver, Sender};
use lazy_static::lazy_static;
use rustyline::{error::ReadlineError, history::FileHistory, Config, DefaultEditor, ExternalPrinter};

use crate::network::runtime::RuntimePlugin;

use super::console_sender::{Console, ConsoleSender};

pub const _REGEX_COMMAND: &str = r####"([\d\w$&+,:;=?@#|'<>.^*()%!-]+)|"([\d\w$&+,:;=?@#|'<>.^*()%!\- ]+)""####;

lazy_static! {
    static ref CONSOLE_OUTPUT_CHANNEL: (Sender<String>, Receiver<String>) = flume::unbounded();
    static ref CONSOLE_INPUT_CHANNEL: (Sender<String>, Receiver<String>) = flume::unbounded();
}

#[derive(Resource)]
pub struct ConsoleHandler {}

/// Read and write console std
impl ConsoleHandler {
    pub fn new() -> Self {
        ConsoleHandler {}
    }

    pub fn run_handler() {
        let config = Config::builder()
            .history_ignore_space(true)
            .auto_add_history(true)
            .build();
        let history = FileHistory::with_config(config);

        let mut rl = DefaultEditor::with_history(config, history).unwrap();
        let mut printer = rl.create_external_printer().unwrap();

        thread::spawn(move || loop {
            let console = Console::init();

            let readline = rl.readline("");
            match readline {
                Ok(input) => {
                    if input.len() > 0 {
                        ConsoleHandler::execute_command(&console, input);
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    RuntimePlugin::stop_server();
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
        CONSOLE_OUTPUT_CHANNEL.0.send(message).unwrap();
    }

    pub fn update(printer: &mut dyn ExternalPrinter) {
        for message in CONSOLE_OUTPUT_CHANNEL.1.try_iter() {
            let date = Local::now();
            printer
                .print(format!("{}: {}", date.format("%Y-%m-%d %H:%M:%S"), message))
                .unwrap();
        }
    }

    pub fn execute_command(sender: &dyn ConsoleSender, message: String) {
        sender.send_console_message(format!("Command \"{}\" not found", message));
    }
}
