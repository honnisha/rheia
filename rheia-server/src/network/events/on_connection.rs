use bevy::prelude::Event;
use bevy_ecs::prelude::EventReader;

use crate::network::client_network::ClientNetwork;

#[derive(Event)]
pub struct PlayerConnectionEvent {
    client: ClientNetwork,
}

impl PlayerConnectionEvent {
    pub fn new(client: ClientNetwork) -> Self {
        Self { client }
    }
}

pub fn on_connection(mut connection_events: EventReader<PlayerConnectionEvent>) {
    for event in connection_events.read() {
        event.client.send_allow_connection();
    }
}
