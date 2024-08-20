use bevy::prelude::Event;
use bevy_ecs::prelude::EventReader;

use crate::network::clients_container::ClientCell;

#[derive(Event)]
pub struct PlayerConnectionEvent {
    client: ClientCell,
}

impl PlayerConnectionEvent {
    pub fn new(client: ClientCell) -> Self {
        Self { client }
    }
}

pub fn on_connection(mut connection_events: EventReader<PlayerConnectionEvent>) {
    for event in connection_events.read() {
        let client = event.client.read();
        client.send_allow_connection();
    }
}
