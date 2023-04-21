use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread, time::Duration,
};

use crate::{console::console_handler::Console, network::server::NetworkServer};
use clap::Parser;

mod console;
mod network;
use lazy_static::lazy_static;
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

lazy_static! {
    pub static ref SERVER_ACTIVE: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
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

    let server_active = SERVER_ACTIVE.clone();
    thread::spawn(move || loop {
        let readline = rl.readline("");
        match readline {
            Ok(input) => {
                Console::input(input);
            }
            Err(ReadlineError::Interrupted) => {
                server_active.store(false, Ordering::Relaxed);
                break;
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    });

    thread::spawn(move || loop {
        Console::update(&mut printer);
        thread::sleep(Duration::from_millis(50));
    });

    println!("HonnyCraft Server version {}", VERSION);
    let ip_port = format!("{}:{}", args.ip, args.port);

    let mut server = NetworkServer::init(ip_port);
    loop {
        if SERVER_ACTIVE.load(Ordering::Relaxed) {
            server.update();
        } else {
            server.stop();
            break;
        }
    }
}
