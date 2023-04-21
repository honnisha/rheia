use std::thread;

use crate::{console::console_handler::Console, network::server::NetworkServer};
use clap::Parser;

mod console;
mod network;
use rustyline::{error::ReadlineError, history::FileHistory, Config, DefaultEditor};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct MainCommand {
    #[arg(short, long, default_value_t = String::from("127.0.0.1"))]
    ip: String,

    #[arg(short, long, default_value_t = String::from("14191"))]
    port: String,
}

fn main() {
    let args = MainCommand::parse();

    let config = Config::builder()
        .history_ignore_space(true)
        .auto_add_history(true)
        .build();
    let history = FileHistory::with_config(config);

    let mut rl = DefaultEditor::with_history(config, history).unwrap();
    let mut printer = rl.create_external_printer().unwrap();

    thread::spawn(move || loop {
        let readline = rl.readline("");
        match readline {
            Ok(input) => {
                Console::input(input);
            }
            Err(ReadlineError::Interrupted) => {
                println!("Interrupted");
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    });

    println!("HonnyCraft Server version {}", VERSION);
    let ip_port = format!("{}:{}", args.ip, args.port);

    let mut server = NetworkServer::init(ip_port);
    loop {
        server.update();
        Console::update(&mut printer);
    }
}
