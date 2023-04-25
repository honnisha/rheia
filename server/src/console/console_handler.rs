use chrono::Local;
use crossbeam_channel::{unbounded, Receiver, Sender};
use lazy_static::lazy_static;
use rustyline::ExternalPrinter;

lazy_static! {
    static ref CONSOLE_OUTPUT_CHANNEL: (Sender<String>, Receiver<String>) = unbounded();
    static ref CONSOLE_INPUT_CHANNEL: (Sender<String>, Receiver<String>) = unbounded();
}

pub trait ConsoleSender {
    fn get_name(&self) -> &String;
    fn send_console_message(&self, message: String);
}

pub struct Console {
    name: String,
}

impl Console {
    pub fn init() -> Self {
        Console {
            name: "Console".to_string(),
        }
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
}

impl ConsoleSender for Console {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn send_console_message(&self, message: String) {
        Console::send_message(message)
    }
}

pub struct ConsoleHandler {}

impl ConsoleHandler {
    pub fn init() -> Self {
        ConsoleHandler {}
    }

    pub fn execute_command(&self, sender: &dyn ConsoleSender, _message: String) {
        sender.send_console_message("Command not found".to_string());
    }
}
