use bevy::prelude::Event;
use bevy_ecs::prelude::EventReader;
use bevy_ecs::system::Res;
use log::info;

use crate::entities::entity::{Position, Rotation};
use crate::network::client_network::ClientInfo;
use crate::network::clients_container::ClientsContainer;
use crate::worlds::worlds_manager::WorldsManager;
use crate::{client_resources::resources_manager::ResourceManager, network::server::NetworkPlugin};

#[derive(Event)]
pub struct PlayerConnectionInfoEvent {
    client_id: u64,
    login: String,
}

impl PlayerConnectionInfoEvent {
    pub fn new(client_id: u64, login: String) -> Self {
        Self { client_id, login }
    }
}

pub fn on_connection_info(
    mut connection_info_events: EventReader<PlayerConnectionInfoEvent>,
    resources_manager: Res<ResourceManager>,
    clients: Res<ClientsContainer>,
    worlds_manager: Res<WorldsManager>,
) {
    for event in connection_info_events.iter() {
        let mut client = clients.get_mut(&event.client_id);
        client.set_client_info(ClientInfo::new(event.login.clone()));

        info!(
            "Connected ip:{} login:{} id:{}",
            client.get_client_ip(),
            client.get_client_info().unwrap().get_login(),
            client.get_client_id()
        );
        NetworkPlugin::send_resources(&client, &resources_manager);

        let default_world = "default".to_string();
        if worlds_manager.has_world_with_slug(&default_world) {
            let position = Position::new(0.0, 60.0, 0.0);
            let rotation = Rotation::new(0.0, 0.0);

            let mut world_manager = worlds_manager.get_world_manager_mut(&default_world).unwrap();
            let world_entity = world_manager.spawn_player(client.get_client_id(), position, rotation);
            client.set_world_entity(Some(world_entity));

            client.network_send_teleport(&position, &rotation);
        }
    }
}
