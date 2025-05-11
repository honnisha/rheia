pub mod messages;
pub mod client;
pub mod server;
pub mod entities;

#[cfg(feature = "network-renet")]
pub mod renet;

#[cfg(feature = "network-renet")]
pub type NetworkClient = renet::client::RenetClientNetwork;

#[cfg(feature = "network-renet")]
pub type NetworkServer = renet::server::RenetServerNetwork;

#[cfg(feature = "network-renet")]
pub type NetworkServerConnection = renet::server::RenetServerConnection;

#[cfg(feature = "network-tokio")]
pub mod tokio;

#[cfg(feature = "network-tokio")]
pub type NetworkClient = tokio::client::TokioClient;

#[cfg(feature = "network-tokio")]
pub type NetworkServer = tokio::server::TokioServer;

#[cfg(feature = "network-tokio")]
pub type NetworkServerConnection = tokio::server::TokioServerConnection;
