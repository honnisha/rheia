use bevy::prelude::{Event, EventWriter};
use bevy_ecs::prelude::EventReader;
use bevy_ecs::system::Res;
use network::messages::{NetworkMessageType, ServerMessages};

use crate::client_resources::resources_manager::ResourceManager;
use crate::client_resources::resources_manager::ARCHIVE_CHUNK_SIZE;
use crate::network::client_network::ClientInfo;
use crate::network::client_network::ClientNetwork;
use crate::network::events::on_media_loaded::PlayerMediaLoadedEvent;

#[derive(Event)]
pub struct PlayerConnectionInfoEvent {
    client: ClientNetwork,
    pub login: String,
    pub version: String,
    pub architecture: String,
    pub rendering_device: String,
}

impl PlayerConnectionInfoEvent {
    pub fn new(
        client: ClientNetwork,
        login: String,
        version: String,
        architecture: String,
        rendering_device: String,
    ) -> Self {
        Self {
            client,
            login,
            version,
            architecture,
            rendering_device,
        }
    }
}

pub fn on_connection_info(
    mut connection_info_events: EventReader<PlayerConnectionInfoEvent>,
    resources_manager: Res<ResourceManager>,
    mut player_media_loaded_events: EventWriter<PlayerMediaLoadedEvent>,
) {
    for event in connection_info_events.read() {
        event.client.set_client_info(ClientInfo::new(&event));

        let client_info = event.client.get_client_info().unwrap();
        log::info!(
            target: "network",
            "Connected ip:&e{}&r login:&a{}&r id:&e{}&r version:&e{}&r os:&e{}",
            event.client.get_client_ip(),
            client_info.get_login(),
            event.client.get_client_id(),
            client_info.get_version(),
            client_info.get_architecture(),
        );

        if resources_manager.has_any_resources() {
            // Start sending media if necessary

            let scheme = ServerMessages::ResourcesScheme {
                list: resources_manager.get_resources_scheme().clone(),
                archive_hash: resources_manager.get_archive_hash().clone(),
            };
            event.client.send_message(NetworkMessageType::ReliableOrdered, &scheme);

            let total = resources_manager.get_archive_parts_count(ARCHIVE_CHUNK_SIZE);
            let resources_part = ServerMessages::ResourcesPart {
                index: 0,
                total: total as u32,
                data: resources_manager.get_archive_part(0, ARCHIVE_CHUNK_SIZE),
            };
            event.client.send_message(NetworkMessageType::ReliableUnordered, &resources_part);
        } else {
            // Or send player as loaded

            let msg = PlayerMediaLoadedEvent::new(event.client.clone(), None);
            player_media_loaded_events.write(msg);
        }
    }
}
