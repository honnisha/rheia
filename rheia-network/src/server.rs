#![allow(opaque_hidden_inferred_bound)]

use std::{future::Future, time::Duration};

use super::messages::{ClientMessages, NetworkMessageType, ServerMessages};

pub trait IServerNetwork<C: IServerConnection> {
    fn new(ip_port: String) -> impl Future<Output = Self>;
    fn step(&self, delta: Duration) -> impl Future<Output = ()>;

    fn drain_connections(&self) -> impl Iterator<Item = ConnectionMessages<C>>;
    fn drain_errors(&self) -> impl Iterator<Item = String>;
    fn is_connected(&self, connection: &C) -> bool;
}

pub enum ConnectionMessages<C: IServerConnection> {
    Connect { connection: C },
    Disconnect { client_id: u64, reason: String },
}

pub trait IServerConnection: Clone {
    fn get_ip(&self) -> &String;
    fn get_client_id(&self) -> u64;
    fn drain_client_messages(&self) -> impl Iterator<Item = ClientMessages>;
    fn send_message(&self, message_type: NetworkMessageType, message: &ServerMessages);
    fn disconnect(&mut self);
}
