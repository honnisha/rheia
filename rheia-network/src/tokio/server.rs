use std::{sync::Arc, time::Duration};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::RwLock,
};

use crate::{
    messages::{ClientMessages, NetworkMessageType, ServerMessages},
    server::{ConnectionMessages, IServerConnection, IServerNetwork},
};

pub struct TokioServer {
    listener: TcpListener,

    channel_connections: (
        flume::Sender<ConnectionMessages<TokioServerConnection>>,
        flume::Receiver<ConnectionMessages<TokioServerConnection>>,
    ),
    channel_errors: (flume::Sender<String>, flume::Receiver<String>),
}

impl IServerNetwork<TokioServerConnection> for TokioServer {
    async fn new(ip_port: String) -> Self {
        let listener = TcpListener::bind(&ip_port).await.unwrap();
        let result = Self {
            listener,
            channel_connections: flume::unbounded(),
            channel_errors: flume::unbounded(),
        };
        return result;
    }

    async fn step(&self, _delta: Duration) {
        let (socket, addr) = self.listener.accept().await.unwrap();

        let mut connection = TokioServerConnection::create(socket, addr.to_string());
        let connect = ConnectionMessages::Connect {
            connection: connection.clone(),
        };
        self.channel_connections.0.send(connect).unwrap();

        tokio::spawn(async move {
            loop {
                if !connection.step().await {
                    break;
                }
            }
        });
    }

    fn drain_connections(&self) -> impl Iterator<Item = ConnectionMessages<TokioServerConnection>> {
        self.channel_connections.1.drain()
    }

    fn drain_errors(&self) -> impl Iterator<Item = String> {
        self.channel_errors.1.drain()
    }

    fn is_connected(&self, _connection: &TokioServerConnection) -> bool {
        todo!()
    }
}

#[derive(Clone)]
pub struct TokioServerConnection {
    socket: Arc<RwLock<TcpStream>>,
    client_id: u64,
    ip: String,

    channel_client_messages: (
        flume::Sender<(u64, ClientMessages)>,
        flume::Receiver<(u64, ClientMessages)>,
    ),
}

impl TokioServerConnection {
    fn create(socket: TcpStream, ip: String) -> Self {
        Self {
            socket: Arc::new(RwLock::new(socket)),
            client_id: 0,
            ip,

            channel_client_messages: flume::unbounded(),
        }
    }

    async fn step(&mut self) -> bool {
        let mut socket = self.socket.write().await;

        let mut buf = vec![0; 1024];
        let n = socket.read(&mut buf).await.expect("failed to read data from socket");
        if n == 0 {
            return false;
        }

        socket
            .write_all(&buf[0..n])
            .await
            .expect("failed to write data to socket");
        true
    }
}

impl IServerConnection for TokioServerConnection {
    fn get_ip(&self) -> &String {
        &self.ip
    }

    fn get_client_id(&self) -> u64 {
        self.client_id
    }

    fn drain_client_messages(&self) -> impl Iterator<Item = (u64, ClientMessages)> {
        self.channel_client_messages.1.drain()
    }

    fn send_message(&self, _message_type: NetworkMessageType, _message: &ServerMessages) {
        todo!();
    }
}
