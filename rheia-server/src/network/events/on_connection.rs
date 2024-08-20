use bevy::prelude::Event;
use bevy_ecs::prelude::EventReader;
use bevy_ecs::system::Res;

use crate::network::clients_container::ClientsContainer;

#[derive(Event)]
pub struct PlayerConnectionEvent {
    client_id: u64,
}

impl PlayerConnectionEvent {
    pub fn new(client_id: u64) -> Self {
        Self { client_id }
    }
}

pub fn on_connection(mut connection_events: EventReader<PlayerConnectionEvent>, clients: Res<ClientsContainer>) {
    for event in connection_events.read() {
        let client = clients.get(&event.client_id).unwrap().read();
        client.allow_connection();
    }
}
