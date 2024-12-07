use bevy::prelude::{Event, EventWriter};
use bevy::prelude::{Events, ResMut};
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
    mut connection_info_events: ResMut<Events<PlayerConnectionInfoEvent>>,
    resources_manager: Res<ResourceManager>,
    mut player_media_loaded_events: EventWriter<PlayerMediaLoadedEvent>,
) {
    for mut event in connection_info_events.drain() {
        event.client.set_client_info(ClientInfo::new(&event));

        let client_info = event.client.get_client_info().unwrap();
        log::info!(
            target: "network",
            "Connected ip:{} login:{} id:{} version:{} os:{}",
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
            let is_last = 1 >= total;
            let resources_part = ServerMessages::ResourcesPart {
                index: 0,
                total: total as u32,
                data: resources_manager.get_archive_part(0, ARCHIVE_CHUNK_SIZE),
                last: is_last,
            };
            event
                .client
                .send_message(NetworkMessageType::ReliableUnordered, &resources_part);
        } else {
            // Or send player as loaded

            let msg = PlayerMediaLoadedEvent::new(event.client.clone(), None);
            player_media_loaded_events.send(msg);
        }
    }
}
