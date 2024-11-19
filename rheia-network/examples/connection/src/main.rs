use clap::Parser;
use clinets::Client;
use console::Console;
use log::LevelFilter;
use logger::CONSOLE_LOGGER;
use server::Server;
use std::error::Error;

pub mod clinets;
pub mod console;
pub mod logger;
pub mod server;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("192.168.0.185:25565"))]
    ip: String,

    #[arg(short, long, default_value_t = String::from("test"))]
    login: String,

    /// Number of times to greet
    #[arg(short = 't', long, default_value_t = String::from("client"))]
    run_type: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    log::set_max_level(LevelFilter::Info);
    log::set_logger(&CONSOLE_LOGGER).unwrap();

    Console::create();

    let args = Args::parse();
    log::info!("args: {:?}", args);

    if args.run_type == "server".to_string() {
        let mut server = Server::create(args.ip.clone()).await;
        server.run().await;
    } else {
        let mut client = Client::create(args.ip.clone(), args.login).await;
        client.run().await;
    }
    Ok(())
}
