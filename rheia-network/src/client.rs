#![allow(opaque_hidden_inferred_bound)]

use super::messages::{ClientMessages, NetworkMessageType, ServerMessages};
use flume::Drain;
use parking_lot::RwLockReadGuard;
use std::{future::Future, net::SocketAddr, time::Duration};
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    TokioAsyncResolver,
};

#[derive(Default, Clone)]
pub struct NetworkInfo {
    pub is_disconnected: bool,
    pub bytes_received_per_sec: f64,
    pub bytes_sent_per_sec: f64,
    pub packet_loss: f64,
}

pub trait IClientNetwork: Sized {
    fn new(ip_port: String) -> impl Future<Output = Result<Self, String>>;
    fn step(&self, delta: Duration) -> impl Future<Output = bool> + Send;

    fn iter_server_messages(&self) -> Drain<'_, ServerMessages>;
    fn iter_errors(&self) -> Drain<'_, String>;

    fn is_connected(&self) -> bool;

    fn disconnect(&self);

    fn send_message(&self, message_type: NetworkMessageType, message: &ClientMessages);

    fn get_network_info(&self) -> RwLockReadGuard<'_, NetworkInfo>;
}

pub async fn resolve_connect_domain(input: &String, default_port: u16) -> Result<SocketAddr, String> {
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

    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());
    let response = resolver.lookup_ip(domain).await.unwrap();

    // let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    // resolver.lookup_ip(domain).unwrap()

    let address = response.iter().next().expect("no addresses returned!");
    if address.is_ipv6() {
        return Err("ipv6 is not supported".to_string());
    }

    Ok(SocketAddr::new(address, port))
}

pub fn resolve_connect_domain_sync(input: &String, default_port: u16) -> Result<SocketAddr, String> {
    let io_loop = tokio::runtime::Runtime::new().unwrap();
    let result = io_loop.block_on(async { resolve_connect_domain(input, default_port) });
    io_loop.block_on(result)
}
