use bevy::prelude::{Event, Res};
use bevy_ecs::prelude::EventReader;
use network::messages::{NetworkMessageType, ServerMessages};

use crate::{client_resources::server_settings::ServerSettings, network::clients_container::ClientCell};

#[derive(Event)]
pub struct PlayerMediaLoadedEvent {
    client: ClientCell,
}

impl PlayerMediaLoadedEvent {
    pub fn new(client: ClientCell) -> Self {
        Self { client }
    }
}

pub fn on_media_loaded(mut events: EventReader<PlayerMediaLoadedEvent>, server_settings: Res<ServerSettings>) {
    for event in events.read() {
        let client = event.client.read();
        let msg = ServerMessages::Settings {
            block_types: server_settings.get_block_types().clone(),
        };
        client.send_message(NetworkMessageType::ReliableOrdered, msg);
    }
}
