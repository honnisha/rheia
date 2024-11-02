use bevy::prelude::{Event, Res};
use bevy_ecs::prelude::EventReader;
use network::messages::{NetworkMessageType, ServerMessages};

use crate::{
    client_resources::{
        resources_manager::{ResourceManager, ARCHIVE_CHUNK_SIZE},
        server_settings::ServerSettings,
    },
    network::clients_container::ClientCell,
};

#[derive(Event)]
pub struct PlayerMediaLoadedEvent {
    client: ClientCell,
    last_index: Option<u32>,
}

/// Event to confirm data download
///
/// last_index is last downloaded index part
impl PlayerMediaLoadedEvent {
    pub fn new(client: ClientCell, last_index: Option<u32>) -> Self {
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
                        last: is_last,
                    };

                    let client = event.client.read();
                    client.send_message(NetworkMessageType::ReliableUnordered, resources_part);
                    return;
                }
            }
            None => (),
        }

        // Send server settings
        let client = event.client.read();
        let msg = ServerMessages::Settings {
            block_types: server_settings.get_block_types().clone(),
        };
        client.send_message(NetworkMessageType::ReliableOrdered, msg);
    }
}
