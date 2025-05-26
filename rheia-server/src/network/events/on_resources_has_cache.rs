use bevy::prelude::{Event, EventWriter};
use bevy_ecs::prelude::EventReader;
use bevy_ecs::system::Res;
use network::messages::{NetworkMessageType, ServerMessages};

use crate::client_resources::resources_manager::ARCHIVE_CHUNK_SIZE;
use crate::client_resources::resources_manager::ResourceManager;
use crate::network::client_network::ClientNetwork;
use crate::network::events::on_media_loaded::PlayerMediaLoadedEvent;

#[derive(Event)]
pub struct ResourcesHasCacheEvent {
    client: ClientNetwork,
    exists: bool,
}

impl ResourcesHasCacheEvent {
    pub fn new(client: ClientNetwork, exists: bool) -> Self {
        Self { client, exists }
    }
}

pub fn on_resources_has_cache(
    mut events: EventReader<ResourcesHasCacheEvent>,
    resources_manager: Res<ResourceManager>,
    mut player_media_loaded_events: EventWriter<PlayerMediaLoadedEvent>,
) {
    for event in events.read() {
        if !event.exists {
            if resources_manager.has_any_resources() {
                // Send first part of archive
                let total = resources_manager.get_archive_parts_count(ARCHIVE_CHUNK_SIZE);
                let resources_part = ServerMessages::ResourcesPart {
                    index: 0,
                    total: total as u32,
                    data: resources_manager.get_archive_part(0, ARCHIVE_CHUNK_SIZE),
                };
                event
                    .client
                    .send_message(NetworkMessageType::ReliableUnordered, &resources_part);
                return;
            }
        }
        // Or send player as loaded
        let msg = PlayerMediaLoadedEvent::new(event.client.clone(), None);
        player_media_loaded_events.write(msg);
    }
}
