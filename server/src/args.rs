use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct MainCommand {
    #[arg(short, long, default_value_t = String::from("127.0.0.1"))]
    pub ip: String,

    #[arg(short, long, default_value_t = String::from("14191"))]
    pub port: String,

    #[arg(short, long, default_value_t = 20)]
    pub tick_rate: u32,
}
