use flume::{Drain, Receiver, Sender};
use log::error;
use parking_lot::RwLock;
use parking_lot::RwLockReadGuard;
use rak_rs::client::{Client, DEFAULT_MTU};
use rak_rs::protocol::reliability::Reliability;
use std::net::ToSocketAddrs;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::network::client::NetworkInfo;
use crate::network::{
    client::ClientNetwork,
    messages::{ClientMessages, NetworkMessageType, ServerMessages},
};

type ClientMessageType = (ClientMessages, NetworkMessageType);

#[derive(Clone)]
pub struct RakNetClientNetwork {
    network_info: Arc<RwLock<NetworkInfo>>,
    network_server_messages: (Sender<ServerMessages>, Receiver<ServerMessages>),
    network_client_messages: (Sender<ClientMessageType>, Receiver<ClientMessageType>),
    network_errors_out: (Sender<String>, Receiver<String>),
    connected: Arc<AtomicBool>,
    disconnect: (Sender<bool>, Receiver<bool>),
}

impl RakNetClientNetwork {
    fn get_reliability(message_type: NetworkMessageType) -> Reliability {
        match message_type {
            NetworkMessageType::ReliableOrdered => Reliability::ReliableSeq,
            NetworkMessageType::ReliableUnordered => Reliability::Reliable,
            NetworkMessageType::Unreliable => Reliability::Unreliable,
        }
    }

    pub fn set_connected(&self, state: bool) {
        self.connected.store(state, Ordering::Relaxed);
    }
}

impl ClientNetwork for RakNetClientNetwork {
    fn new(ip_port: String, login: String) -> Result<Self, String> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        let network = RakNetClientNetwork {
            network_info: Arc::new(RwLock::new(Default::default())),
            network_server_messages: flume::unbounded(),
            network_client_messages: flume::unbounded(),
            network_errors_out: flume::unbounded(),
            connected: Arc::new(AtomicBool::new(false)),
            disconnect: flume::bounded(1),
        };
        runtime.block_on(async {
            let network = network.clone();
            let mut client = Client::new(1, DEFAULT_MTU);
            let mut addr = ip_port.to_socket_addrs().unwrap();
            if let Err(e) = client.connect(addr.next().unwrap()).await {
                network
                    .network_errors_out
                    .0
                    .send(format!("Failed to connect to server: {:?}", e))
                    .unwrap();
                return;
            }
            network.set_connected(true);

            let connection_info = ClientMessages::ConnectionInfo { login };
            network.send_message(&connection_info, NetworkMessageType::ReliableOrdered);

            // Messages reciever
            loop {
                for _ in network.disconnect.1.drain() {
                    break;
                }

                let encoded = match client.recv().await {
                    Ok(e) => e,
                    Err(_) => {
                        error!("Serer recv message error");
                        continue;
                    }
                };
                let decoded: ServerMessages = match bincode::deserialize(&encoded) {
                    Ok(d) => d,
                    Err(e) => {
                        error!("Decode server message error: {}", e);
                        continue;
                    }
                };
                network.network_server_messages.0.send(decoded).unwrap();

                for (message, message_type) in network.network_client_messages.1.drain() {
                    let encoded = bincode::serialize(&message).unwrap();
                    client
                        .send(&encoded, RakNetClientNetwork::get_reliability(message_type), 0)
                        .await
                        .unwrap();
                }
            }
        });

        // Messages sender
        Ok(network)
    }

    fn iter_server_messages(&self) -> Drain<ServerMessages> {
        self.network_server_messages.1.drain()
    }

    fn iter_errors(&self) -> Drain<String> {
        self.network_errors_out.1.drain()
    }

    fn is_connected(&self) -> bool {
        self.connected.load(Ordering::Relaxed)
    }

    fn disconnect(&self) {
        self.disconnect.0.send(true).unwrap();
        self.set_connected(false);
    }

    fn send_message(&self, message: &ClientMessages, message_type: NetworkMessageType) {
        self.network_client_messages
            .0
            .send((message.clone(), message_type))
            .unwrap();
    }

    fn get_network_info(&self) -> RwLockReadGuard<NetworkInfo> {
        self.network_info.read()
    }
}
