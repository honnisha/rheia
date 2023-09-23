use bevy::prelude::Event;
use bevy_ecs::prelude::EventReader;
use bevy_ecs::system::Res;
use log::info;

use crate::network::clients_container::ClientsContainer;
use crate::network::server::NetworkContainer;
use crate::{client_resources::resources_manager::ResourceManager};

#[derive(Event)]
pub struct PlayerConnectionEvent {
    client_id: u64,
    ip: String,
}

impl PlayerConnectionEvent {
    pub fn new(client_id: u64, ip: String) -> Self {
        Self { client_id, ip }
    }
}

pub fn on_connection(
    network_container: Res<NetworkContainer>,
    mut connection_events: EventReader<PlayerConnectionEvent>,
    resources_manager: Res<ResourceManager>,
    clients: Res<ClientsContainer>,
) {
    for event in connection_events.iter() {
        let mut client = clients.get_mut(&event.client_id);
    }
}
