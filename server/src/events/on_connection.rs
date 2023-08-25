use bevy::prelude::Event;
use bevy_ecs::prelude::EventReader;
use bevy_ecs::system::Res;
use log::info;

use crate::entities::entity::{Position, Rotation};
use crate::network::clients_container::ClientsContainer;
use crate::network::server::NetworkContainer;
use crate::worlds::worlds_manager::WorldsManager;
use crate::{client_resources::resources_manager::ResourceManager, network::server::NetworkPlugin};

#[derive(Event)]
pub struct PlayerConnectionEvent {
    client_id: u64,
}

impl PlayerConnectionEvent {
    pub fn new(client_id: u64) -> Self {
        Self { client_id }
    }
}

pub fn on_connection(
    network_container: Res<NetworkContainer>,
    mut connection_events: EventReader<PlayerConnectionEvent>,
    resources_manager: Res<ResourceManager>,
    clients: Res<ClientsContainer>,
    worlds_manager: Res<WorldsManager>,
) {
    for event in connection_events.iter() {
        let mut client = clients.get_mut(&event.client_id);
        info!("Connected login \"{}\"", client.get_login());
        NetworkPlugin::send_resources(&event.client_id, &resources_manager);

        let default_world = "default".to_string();
        if worlds_manager.has_world_with_slug(&default_world) {
            let position = Position::new(0.0, 60.0, 0.0);
            let rotation = Rotation::new(0.0, 0.0);

            {
                let mut world_manager = worlds_manager.get_world_manager_mut(&default_world).unwrap();
                let world_entity = world_manager.spawn_player(client.get_client_id(), position, rotation);
                client.world_entity = Some(world_entity);
            }

            client.send_teleport(&network_container, &position, &rotation);
            client.send_already_loaded_chunks(&network_container, &worlds_manager);
        }
    }
}
