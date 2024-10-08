use bevy::prelude::{Commands, Event, EventWriter};
use bevy_ecs::prelude::EventReader;
use bevy_ecs::system::Res;
use network::messages::{NetworkMessageType, ServerMessages};

use crate::network::client_network::ClientInfo;
use crate::network::clients_container::ClientCell;
use crate::network::events::on_media_loaded::PlayerMediaLoadedEvent;
use crate::worlds::worlds_manager::WorldsManager;
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

        let media_count = resources_manager.get_media_count();

        if media_count > 0 {
            // Start sending media if necessary

            let msg = ServerMessages::MediaCount {
                count: media_count.clone(),
            };
            client.send_message(NetworkMessageType::ReliableOrdered, msg);
            NetworkPlugin::send_resources(&client, &resources_manager);
        } else {
            // Or send player as loaded

            let msg = PlayerMediaLoadedEvent::new(event.client.clone());
            player_media_loaded_events.send(msg);
        }
    }
}
