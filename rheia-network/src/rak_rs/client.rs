use crate::network::client::NetworkInfo;
use crate::network::{
    client::ClientNetwork,
    messages::{ClientMessages, NetworkMessageType, ServerMessages},
};
use flume::{Drain, Receiver, Sender};
use parking_lot::RwLock;
use parking_lot::RwLockReadGuard;
use rak_rs::client::{Client, DEFAULT_MTU};
use rak_rs::protocol::reliability::Reliability;
use std::net::ToSocketAddrs;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use tokio::time::{sleep, Duration};

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
    fn new(ip_port: String) -> Result<Self, String> {
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

        let mut client = Client::new(11, DEFAULT_MTU);

        let n = network.clone();

        thread::spawn(move || {
            runtime.block_on(async {
                log::debug!(target: "raknet", "Network thread spawned successfully");

                match Client::ping(ip_port.clone()).await {
                    Ok(s) => log::debug!(target: "raknet", "Servet ping successfully: {:?}", s),
                    Err(e) => log::error!(target: "raknet", "Failed to ping server: {}", e),
                }

                log::debug!(target: "raknet", "Connection to {}...", ip_port);

                let mut addr = ip_port.clone().to_socket_addrs().unwrap();
                if let Err(e) = client.connect(addr.next().unwrap()).await {
                    let err_sender = n.network_errors_out.0;
                    let err = format!("Failed to connect to {} server: {:?}", ip_port, e);
                    log::debug!(target: "raknet", "Error: {}", err);
                    err_sender.send(err).unwrap();
                    return;
                }
                n.set_connected(true);
                log::debug!(target: "raknet", "Connected successfully");

                loop {
                    for _ in n.disconnect.1.drain() {
                        log::error!(target: "raknet", "Disconnect recieved");
                        break;
                    }

                    match client.recv().await {
                        Ok(encoded) => {
                            let decoded: ServerMessages = match bincode::deserialize(&encoded) {
                                Ok(d) => d,
                                Err(e) => {
                                    log::error!(target: "raknet", "Decode server message error: \"{}\" original: {:?}", e, encoded);
                                    continue;
                                }
                            };
                            log::debug!(target: "raknet", "RECIEVED message: {:?}", decoded);
                            n.network_server_messages.0.send(decoded).unwrap();
                        },
                        Err(_) => {
                            log::error!(target: "raknet", "Serer recv message error");
                            continue;
                        }
                    };

                    for (message, message_type) in n.network_client_messages.1.drain() {
                        let encoded = bincode::serialize(&message).unwrap();
                        log::debug!(target: "raknet", "SEND message {:?} encoded: {:?}", message, encoded);
                        client
                            .send(&encoded, RakNetClientNetwork::get_reliability(message_type), 0)
                            .await
                            .unwrap();
                    }
                    sleep(Duration::from_millis(50)).await;
                }
            });
        });
        log::debug!(target: "raknet", "RakNetServerNetwork thread created");
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
