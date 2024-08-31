pub mod messages;
pub mod client;
pub mod server;

#[cfg(feature = "network-renet")]
pub mod renet;

#[cfg(feature = "network-renet")]
pub type NetworkClient = renet::client::RenetClientNetwork;

#[cfg(feature = "network-renet")]
pub type NetworkServer = renet::server::RenetServerNetwork;


#[cfg(feature = "network-rak-rs")]
pub mod rak_rs;

#[cfg(feature = "network-rak-rs")]
pub type NetworkClient = rak_rs::client::RakNetClientNetwork;

#[cfg(feature = "network-rak-rs")]
pub type NetworkServer = rak_rs::server::RakNetServerNetwork;


#[cfg(feature = "network-steamworks")]
pub mod steamworks;

#[cfg(feature = "network-steamworks")]
pub type NetworkClient = steamworks::client::SteamworksClient;

#[cfg(feature = "network-steamworks")]
pub type NetworkServer = steamworks::server::SteamworksServer;


#[cfg(feature = "network-tokio")]
pub mod tokio;

#[cfg(feature = "network-tokio")]
pub type NetworkClient = tokio::client::TokioClient;

#[cfg(feature = "network-tokio")]
pub type NetworkServer = tokio::server::TokioServer;
