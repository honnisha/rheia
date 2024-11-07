use std::net::SocketAddr;

use super::messages::{ClientMessages, NetworkMessageType, ServerMessages};
use flume::Drain;
use parking_lot::RwLockReadGuard;
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    Resolver,
};

#[derive(Default, Clone)]
pub struct NetworkInfo {
    pub is_disconnected: bool,
    pub bytes_received_per_second: f64,
    pub bytes_received_per_sec: f64,
    pub bytes_sent_per_sec: f64,
    pub packet_loss: f64,
}

pub trait IClientNetwork: Sized {
    fn new(ip_port: String) -> Result<Self, String>;
    // fn step(&self, delta: Duration) -> bool;

    fn iter_server_messages(&self) -> Drain<ServerMessages>;
    fn iter_errors(&self) -> Drain<String>;

    fn is_connected(&self) -> bool;

    fn disconnect(&self);

    fn send_message(&self, message: &ClientMessages, message_type: NetworkMessageType);

    fn get_network_info(&self) -> RwLockReadGuard<NetworkInfo>;
}

pub fn resolve_connect_domain(input: &String, default_port: u16) -> Result<SocketAddr, String> {
    let collection: Vec<&str> = input.split(":").collect();
    let (domain, port) = if collection.len() == 2 {
        (
            collection.get(0).unwrap().to_string(),
            match collection.get(1).unwrap().parse() {
                Ok(p) => p,
                Err(e) => return Err(format!("port error: {}", e)),
            },
        )
    } else {
        (input.clone(), default_port)
    };

    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let response = resolver.lookup_ip(domain).unwrap();
    let address = response.iter().next().expect("no addresses returned!");
    if address.is_ipv6() {
        return Err("ipv6 is not supported".to_string());
    }

    Ok(SocketAddr::new(address, port))
}
