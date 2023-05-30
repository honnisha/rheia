//! Client part of the plugin. You can enable it by adding `client` feature.

use std::future::Future;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::connection::{EcsConnection, RawConnection};
use crate::protocol::ReadStream;
use crate::protocol::WriteStream;
use crate::protocol::{NetworkStream, ReceiveError};
use crate::{ClientConfig, Protocol};

/// Client-side connection to a server.
pub type ClientConnection<Config> = EcsConnection<<Config as ClientConfig>::ClientPacket>;
/// List of client-side connections to a server.

pub struct ClientConnections<Config: ClientConfig>(Vec<ClientConnection<Config>>);
impl<Config: ClientConfig> ClientConnections<Config> {
    fn new() -> Self {
        Self(Vec::new())
    }

    pub fn get_connection(&mut self) -> Option<&ClientConnection<Config>> {
        self.0.first()
    }

    pub fn has_connection(&mut self) -> bool {
        self.0.len() > 0
    }
}

impl<Config: ClientConfig> std::ops::Deref for ClientConnections<Config> {
    type Target = Vec<ClientConnection<Config>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Config: ClientConfig> std::ops::DerefMut for ClientConnections<Config> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[allow(type_alias_bounds)]
pub type ErrorType<Config: ClientConfig> =
    ReceiveError<Config::ServerPacket, Config::ClientPacket, Config::Serializer, Config::LengthSerializer>;

pub struct ClientNetwork<Config: ClientConfig> {
    pub connections: ClientConnections<Config>,
    connection_request_tx: UnboundedSender<SocketAddr>,

    pub connection_receiver_rx: UnboundedReceiver<(SocketAddr, ClientConnection<Config>)>,
    pub disconnection_receiver_rx: UnboundedReceiver<(ErrorType<Config>, SocketAddr)>,
    pub packet_receiver_rx: UnboundedReceiver<(ClientConnection<Config>, Config::ServerPacket)>,
}

impl<Config: ClientConfig> ClientNetwork<Config> {
    pub fn init() -> ClientNetwork<Config> {
        let (req_tx, mut req_rx) = tokio::sync::mpsc::unbounded_channel();

        let (conn_tx, conn_rx) = tokio::sync::mpsc::unbounded_channel();
        let (conn_tx2, mut conn_rx2) = tokio::sync::mpsc::unbounded_channel();
        let (disc_tx, disc_rx) = tokio::sync::mpsc::unbounded_channel();
        let (pack_tx, pack_rx) = tokio::sync::mpsc::unbounded_channel();

        let client = ClientNetwork {
            connections: ClientConnections::new(),
            connection_request_tx: req_tx,

            connection_receiver_rx: conn_rx,
            disconnection_receiver_rx: disc_rx,
            packet_receiver_rx: pack_rx,
        };

        // Connection
        let disc_tx2 = disc_tx.clone();
        run_async(async move {
            while let Some(address) = req_rx.recv().await {
                let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
                match create_connection::<Config>(
                    address,
                    Config::Serializer::default(),
                    Config::LengthSerializer::default(),
                    rx,
                )
                .await
                {
                    Ok(connection) => {
                        let ecs_conn = EcsConnection {
                            disconnect_task: connection.disconnect_task.clone(),
                            id: connection.id(),
                            packet_tx: tx,
                            local_addr: connection.local_addr(),
                            peer_addr: connection.peer_addr(),
                        };
                        conn_tx.send((address, ecs_conn.clone())).unwrap();
                        conn_tx2.send((connection, ecs_conn)).unwrap();
                    }
                    Err(err) => {
                        disc_tx2.send((ReceiveError::NoConnection(err), address)).unwrap();
                    }
                }
            }
        });

        run_async(async move {
            while let Some((connection, ecs_conn)) = conn_rx2.recv().await {
                let RawConnection {
                    disconnect_task,
                    stream,
                    serializer,
                    packet_length_serializer,
                    mut packets_rx,
                    id,
                    _receive_packet,
                    _send_packet,
                } = connection;
                let pack_tx2 = pack_tx.clone();
                let disc_tx2 = disc_tx.clone();
                let serializer2 = Arc::clone(&serializer);
                let packet_length_serializer2 = Arc::clone(&packet_length_serializer);
                let peer_addr = stream.peer_addr();
                let (mut read, mut write) = stream.into_split().await.expect("Couldn't split stream");
                tokio::spawn(async move {
                    loop {
                        tokio::select! {
                            result = read.receive(&*serializer2, &*packet_length_serializer2) => {
                                match result {
                                    Ok(packet) => {
                                        log::trace!("({id:?}) Received packet {packet:?}");
                                        if pack_tx2.send((ecs_conn.clone(), packet)).is_err() {
                                            break
                                        }
                                    }
                                    Err(err) => {
                                        log::debug!("({id:?}) Error receiving next packet: {err:?}");
                                        if disc_tx2.send((err, peer_addr)).is_err() {
                                            break
                                        }
                                        break;
                                    }
                                }
                            }
                            _ = disconnect_task.clone() => {
                                log::debug!("({id:?}) Client disconnected intentionally");
                                disc_tx2.send((ReceiveError::IntentionalDisconnection, peer_addr)).unwrap();
                                break
                            }
                        }
                    }
                });
                // `select!` is not needed because `packets_rx` returns `None` when
                // all senders are be dropped, and `disc_tx2.send(...)` above should
                // remove all senders from ECS.
                tokio::spawn(async move {
                    while let Some(packet) = packets_rx.recv().await {
                        log::trace!("({id:?}) Sending packet {:?}", packet);
                        match write.send(packet, &*serializer, &*packet_length_serializer).await {
                            Ok(()) => (),
                            Err(err) => {
                                log::error!("({id:?}) Error sending packet: {err}");
                                break;
                            }
                        }
                    }
                });
            }
        });
        client
    }

    pub fn connect(&mut self, address: SocketAddr) {
        //if self.max_packet_size.is_none() {
        //    log::warn!("You haven't set \"MaxPacketSize\" resource! This is a security risk, please insert it before using this in production.")
        //}
        //set_max_packet_size_system;

        // Send this event to indicate that you want to connect to a server.
        // Wait for connection_receiver_rx or connection_remove_system to know the connection's state
        self.connection_request_tx.send(address).unwrap();
    }

}

#[allow(clippy::type_complexity)]
pub(crate) async fn create_connection<Config: ClientConfig>(
    addr: SocketAddr,
    serializer: Config::Serializer,
    packet_length_serializer: Config::LengthSerializer,
    packet_rx: UnboundedReceiver<Config::ClientPacket>,
) -> io::Result<
    RawConnection<
        Config::ServerPacket,
        Config::ClientPacket,
        <Config::Protocol as Protocol>::ClientStream,
        Config::Serializer,
        Config::LengthSerializer,
    >,
> {
    Ok(RawConnection::new(
        Config::Protocol::connect_to_server(addr).await?,
        serializer,
        packet_length_serializer,
        packet_rx,
    ))
}

#[cfg(not(target_arch = "wasm32"))]
fn run_async<F>(future: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Cannot start tokio runtime");

        rt.block_on(async move {
            let local = tokio::task::LocalSet::new();
            local
                .run_until(async move {
                    tokio::task::spawn_local(future).await.unwrap();
                })
                .await;
        });
    });
}

#[cfg(target_arch = "wasm32")]
fn run_async<F>(future: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    wasm_bindgen_futures::spawn_local(async move {
        let local = tokio::task::LocalSet::new();
        local
            .run_until(async move {
                tokio::task::spawn_local(future).await.unwrap();
            })
            .await;
    });
}
