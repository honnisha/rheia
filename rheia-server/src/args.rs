use clap::Parser;
use log::LevelFilter;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct MainCommand {
    #[arg(short, long, default_value_t = String::from("127.0.0.1"))]
    pub ip: String,

    #[arg(short, long, default_value_t = String::from("19132"))]
    pub port: String,

    #[arg(long, default_value_t = 512)]
    pub max_packet_size: usize,

    #[arg(long, default_value_t = String::from("info"))]
    pub logs: String,
}

pub(crate) fn get_log_level(level: &String) -> LevelFilter {
    match level.to_lowercase().as_str() {
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "error" => LevelFilter::Error,
        "off" => LevelFilter::Off,
        "trace" => LevelFilter::Trace,
        "warn" => LevelFilter::Warn,
        _ => {
            panic!("Log level \"{}\" not found", level);
        }
    }
}
