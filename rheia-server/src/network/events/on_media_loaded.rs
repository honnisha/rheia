use bevy::prelude::{Event, Res};
use bevy_ecs::prelude::EventReader;
use network::messages::{NetworkMessageType, ServerMessages};

use crate::{
    client_resources::{
        resources_manager::{ARCHIVE_CHUNK_SIZE, ResourceManager},
        server_settings::ServerSettings,
    },
    network::client_network::ClientNetwork,
};

#[derive(Event)]
pub struct PlayerMediaLoadedEvent {
    client: ClientNetwork,
    last_index: Option<u32>,
}

/// Event to confirm data download
///
/// last_index is last downloaded index part
impl PlayerMediaLoadedEvent {
    pub fn new(client: ClientNetwork, last_index: Option<u32>) -> Self {
        Self { client, last_index }
    }
}

pub fn on_media_loaded(
    mut events: EventReader<PlayerMediaLoadedEvent>,
    server_settings: Res<ServerSettings>,
    resources_manager: Res<ResourceManager>,
) {
    for event in events.read() {
        match event.last_index {
            Some(index) => {
                let total = resources_manager.get_archive_parts_count(ARCHIVE_CHUNK_SIZE);
                let is_last = (index as usize) + 1 >= total;
                if !is_last {
                    // Send new media part
                    let resources_part = ServerMessages::ResourcesPart {
                        index: index + 1,
                        total: total as u32,
                        data: resources_manager.get_archive_part(index as usize + 1, ARCHIVE_CHUNK_SIZE),
                    };

                    event
                        .client
                        .send_message(NetworkMessageType::ReliableUnordered, &resources_part);
                    return;
                }
            }
            None => (),
        }

        // Send server settings
        event.client.send_message(
            NetworkMessageType::ReliableOrdered,
            &server_settings.get_network_settings(),
        );
    }
}
