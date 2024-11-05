use bevy::prelude::{Event, EventWriter};
use bevy_ecs::prelude::EventReader;
use bevy_ecs::system::Res;
use common::utils::calculate_hash;
use network::messages::{NetworkMessageType, ServerMessages};

use crate::client_resources::resources_manager::ARCHIVE_CHUNK_SIZE;
use crate::network::client_network::ClientInfo;
use crate::network::clients_container::ClientCell;
use crate::network::events::on_media_loaded::PlayerMediaLoadedEvent;
use crate::{client_resources::resources_manager::ResourceManager, network::server::NetworkPlugin};

#[derive(Event)]
pub struct PlayerConnectionInfoEvent {
    client: ClientCell,
    login: String,
}

impl PlayerConnectionInfoEvent {
    pub fn new(client: ClientCell, login: String) -> Self {
        Self { client, login }
    }
}

pub fn on_connection_info(
    mut connection_info_events: EventReader<PlayerConnectionInfoEvent>,
    resources_manager: Res<ResourceManager>,
    mut player_media_loaded_events: EventWriter<PlayerMediaLoadedEvent>,
) {
    for event in connection_info_events.read() {
        event
            .client
            .write()
            .set_client_info(ClientInfo::new(event.login.clone()));

        let client = event.client.read();

        log::info!(
            target: "network",
            "Connected ip:{} login:{} id:{}",
            client.get_client_ip(),
            client.get_client_info().unwrap().get_login(),
            client.get_client_id()
        );

        if resources_manager.has_any_resources() {
            // Start sending media if necessary

            let scheme = ServerMessages::ResourcesScheme {
                list: resources_manager.get_resources_scheme().clone(),
                archive_hash: resources_manager.get_archive_hash().clone(),
            };
            client.send_message(NetworkMessageType::ReliableOrdered, scheme);

            let total = resources_manager.get_archive_parts_count(ARCHIVE_CHUNK_SIZE);
            let is_last = 1 >= total;
            let resources_part = ServerMessages::ResourcesPart {
                index: 0,
                total: total as u32,
                data: resources_manager.get_archive_part(0, ARCHIVE_CHUNK_SIZE),
                last: is_last,
            };
            client.send_message(NetworkMessageType::ReliableUnordered, resources_part);
        } else {
            // Or send player as loaded

            let msg = PlayerMediaLoadedEvent::new(event.client.clone(), None);
            player_media_loaded_events.send(msg);
        }
    }
}
