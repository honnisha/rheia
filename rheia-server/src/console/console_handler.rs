use bevy_ecs::resource::Resource;
use chrono::Local;
use common::utils::colors::parse_to_terminal_colors;
use flume::{Drain, Receiver, Sender};
use lazy_static::lazy_static;
use rustyline::{
    config::Configurer, error::ReadlineError, highlight::MatchingBracketHighlighter, validate::MatchingBracketValidator, ColorMode, Config, Editor, ExternalPrinter
};
use std::{
    fs::OpenOptions,
    thread::{self},
    time::Duration,
};

use crate::network::runtime_plugin::RuntimePlugin;

use super::{
    completer::{CustomCompleter, CustomHinter},
    helper::CustomHelper,
};

lazy_static! {
    // To handle output log from entire server and draw it on console
    static ref CONSOLE_OUTPUT_CHANNEL: (Sender<String>, Receiver<String>) = flume::unbounded();

    // Console input
    static ref CONSOLE_INPUT_CHANNEL: (Sender<String>, Receiver<String>) = flume::bounded(1);
}

#[derive(Resource, Default)]
pub struct ConsoleHandler {}

const CONSOLE_HISTORY_FILE: &str = "console_history.txt";

/// Read and write console std
impl ConsoleHandler {
    pub fn run_handler(&mut self) {
        let config = Config::builder()
            .history_ignore_space(true)
            .auto_add_history(true)
            .edit_mode(rustyline::EditMode::Emacs)
            .color_mode(ColorMode::Enabled)
            .build();

        let helper = CustomHelper {
            completer: CustomCompleter::default(),
            highlighter: MatchingBracketHighlighter::new(),
            hinter: CustomHinter::default(),
            colored_prompt: "".to_owned(),
            validator: MatchingBracketValidator::new(),
        };

        let mut rl = Editor::with_config(config).unwrap();
        rl.set_helper(Some(helper));
        rl.set_enable_signals(true);

        let _ = OpenOptions::new()
            .create_new(true)
            .write(true)
            .append(true)
            .open(CONSOLE_HISTORY_FILE);

        let _ = match rl.load_history(CONSOLE_HISTORY_FILE) {
            Ok(_) => log::info!(target: "console", "Console file history loaded from &e\"{}\"", CONSOLE_HISTORY_FILE),
            Err(e) => log::error!(target: "console", "Console history &e\"{}\"&r error: &c{}", CONSOLE_HISTORY_FILE, e),
        };

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
                    let _ = match rl.save_history(CONSOLE_HISTORY_FILE) {
                        Ok(_) => {
                            log::info!(target: "console", "Console file history saved in &e\"{}\"", CONSOLE_HISTORY_FILE)
                        }
                        Err(e) => {
                            log::error!(target: "console", "Console file &e\"{}\"&r history save error: &c{}", CONSOLE_HISTORY_FILE, e)
                        }
                    };

                    RuntimePlugin::stop();
                    break;
                }
                Err(e) => {
                    log::error!("Error: {:?}", e);
                }
            }
        });

        thread::spawn(move || loop {
            for message in CONSOLE_OUTPUT_CHANNEL.1.drain() {
                printer.print(message).unwrap();
                thread::sleep(Duration::from_millis(1));
            }
            thread::sleep(Duration::from_millis(50));
        });
    }

    pub fn handle_stop_server(&mut self) {
        for message in CONSOLE_OUTPUT_CHANNEL.1.drain() {
            println!("{}", message);
        }
    }

    pub fn send_message(message: String) {
        let date = Local::now();
        let m = format!(
            "{}: {}",
            date.format("%H:%M:%S.%3f"),
            parse_to_terminal_colors(&message)
        );

        if RuntimePlugin::is_active() {
            CONSOLE_OUTPUT_CHANNEL.0.send(m).unwrap();
        } else {
            println!("{}", m);
        }
    }

    pub fn iter_commands() -> Drain<'static, String> {
        CONSOLE_INPUT_CHANNEL.1.drain()
    }
}
