use bevy::prelude::{Commands, Event};
use bevy_ecs::prelude::EventReader;
use bevy_ecs::system::Res;
use network::messages::{NetworkMessageType, ServerMessages};

use crate::entities::entity::{Position, Rotation};
use crate::network::client_network::ClientInfo;
use crate::network::clients_container::ClientCell;
use crate::worlds::bevy_commands::SpawnPlayer;
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
    mut commands: Commands,
    mut connection_info_events: EventReader<PlayerConnectionInfoEvent>,
    resources_manager: Res<ResourceManager>,
    worlds_manager: Res<WorldsManager>,
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
            // Or spawn player

            let default_world = "default".to_string();
            if !worlds_manager.has_world_with_slug(&default_world) {
                panic!("default world is not found");
            };

            commands.add(SpawnPlayer::create(
                default_world,
                event.client.clone(),
                Position::new(0.0, 30.0, 0.0),
                Rotation::new(0.0, 0.0),
            ));
        }
    }
}
