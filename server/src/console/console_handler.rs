use crossbeam_channel::{unbounded, Receiver, Sender};
use lazy_static::lazy_static;
use rustyline::ExternalPrinter;

pub struct Console {}

lazy_static! {
    pub static ref CONSOLE_OUTPUT_CHANNEL: (Sender<String>, Receiver<String>) = unbounded();
    static ref CONSOLE_INPUT_CHANNEL: (Sender<String>, Receiver<String>) = unbounded();
}

impl Console {
    pub fn send_message(message: String) {
        CONSOLE_OUTPUT_CHANNEL.0.send(message).unwrap();
    }

    pub fn update(printer: &mut dyn ExternalPrinter) {
        for message in CONSOLE_OUTPUT_CHANNEL.1.try_iter() {
            printer.print(message).unwrap();
        }
    }

    pub fn input(message: String) {
    }
}
