use std::time::Duration;
use flume::{Receiver, Sender};
use lazy_static::lazy_static;
use rustyline::ExternalPrinter;

lazy_static! {
    // To handle output log from entire server and draw it on console
    static ref CONSOLE_OUTPUT_CHANNEL: (Sender<String>, Receiver<String>) = flume::unbounded();

    // Console input
    static ref CONSOLE_INPUT_CHANNEL: (Sender<String>, Receiver<String>) = flume::bounded(1);
}

pub struct Console {}

impl Console {
    pub fn create() {
        let mut rl = rustyline::DefaultEditor::new().unwrap();
        let mut printer = rl.create_external_printer().unwrap();
        std::thread::spawn(move || loop {
            let readline = rl.readline("");
            match readline {
                Ok(input) => {
                    if input.len() > 0 {
                        CONSOLE_INPUT_CHANNEL.0.send(input.clone()).unwrap();
                    }
                }
                Err(rustyline::error::ReadlineError::Interrupted) => {
                    log::info!("exit");
                    std::process::exit(1);
                }
                Err(e) => {
                    log::error!("Error: {:?}", e);
                }
            }
        });

        std::thread::spawn(move || loop {
            for message in CONSOLE_OUTPUT_CHANNEL.1.drain() {
                printer.print(message).unwrap();
                std::thread::sleep(Duration::from_millis(1));
            }
            std::thread::sleep(Duration::from_millis(50));
        });
    }

    pub fn send(message: String) {
        CONSOLE_OUTPUT_CHANNEL.0.send(message).unwrap();
    }

    pub fn get_input() -> flume::Drain<'static, String> {
        CONSOLE_INPUT_CHANNEL.1.drain()
    }
}
