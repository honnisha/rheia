use std::{io::{Write, self, Stdout}, thread};

use crate::{console::console_handler::Console, network::server::NetworkServer};
use clap::Parser;

mod console;
mod network;

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

    thread::spawn(move || loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        Console::input(input);
    });

    println!("HonnyCraft Server version {}", VERSION);
    let ip_port = format!("{}:{}", args.ip, args.port);

    let mut server = NetworkServer::init(ip_port);
    loop {
        server.update();
        Console::update();
    }
}
