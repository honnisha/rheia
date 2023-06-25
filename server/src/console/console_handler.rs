use std::{thread, time::Duration};

use bevy::prelude::Resource;
use chrono::Local;
use flume::{Receiver, Sender};
use lazy_static::lazy_static;
use rustyline::{error::ReadlineError, history::FileHistory, Config, DefaultEditor, ExternalPrinter};

use crate::network::runtime_plugin::RuntimePlugin;

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
            .edit_mode(rustyline::EditMode::Emacs)
            .build();
        let history = FileHistory::with_config(config);

        let mut rl = DefaultEditor::with_history(config, history).unwrap();
        let mut printer = rl.create_external_printer().unwrap();

        thread::spawn(move || loop {
            let console = Console::default();

            let readline = rl.readline("");
            match readline {
                Ok(input) => {
                    if input.len() > 0 {
                        ConsoleHandler::execute_command(&console, &input);
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
        }
        else {
            println!("{}", m);
        }
    }

    pub fn update(printer: &mut dyn ExternalPrinter) {
        for message in CONSOLE_OUTPUT_CHANNEL.1.try_iter() {
            printer.print(message).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    }

    pub fn execute_command(sender: &dyn ConsoleSender, command: &String) {
        sender.send_console_message(format!("Command \"{}\" not found", command));
    }
}
