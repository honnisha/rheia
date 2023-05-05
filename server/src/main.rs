use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use crate::{
    client_resources::resources_manager::ResourceManager, console::console_handler::ConsoleHandler,
    network::server::NetworkServer,
};
use clap::Parser;
use lazy_static::lazy_static;
use worlds::worlds_manager::WorldsManager;

mod client_resources;
mod console;
mod network;
mod worlds;

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
    static ref SERVER_ACTIVE: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
}

pub struct HonnyServer {
    worlds_manager: WorldsManager,
    resource_manager: ResourceManager,
    _console_handler: ConsoleHandler,
}

impl HonnyServer {
    pub fn get_resource_manager(&self) -> &ResourceManager {
        &self.resource_manager
    }

    pub fn stop_server() {
        SERVER_ACTIVE.store(false, Ordering::Relaxed);
    }
}

fn main() {
    let args = MainCommand::parse();
    ConsoleHandler::send_message(format!("HonnyCraft Server version {}", VERSION));

    let ip_port = format!("{}:{}", args.ip, args.port);
    let mut server = NetworkServer::init(ip_port);

    let mut honny_server = HonnyServer {
        worlds_manager: WorldsManager::new(),
        resource_manager: ResourceManager::new(),
        _console_handler: ConsoleHandler::new(),
    };
    honny_server.resource_manager.rescan_scripts();

    loop {
        if SERVER_ACTIVE.load(Ordering::Relaxed) {
            server.update(&mut honny_server);
        } else {
            server.stop();

            // Wait console
            thread::sleep(Duration::from_millis(50));
            break;
        }
    }
}
