use bevy::prelude::{Event, EventWriter};
use bevy_ecs::prelude::EventReader;
use bevy_ecs::system::Res;
use common::chunks::block_position::BlockPositionTrait;

use crate::entities::entity::{Position, Rotation};
use crate::network::client_network::ClientInfo;
use crate::network::clients_container::ClientCell;
use crate::network::sync_entities::PlayerSpawnEvent;
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
    worlds_manager: Res<WorldsManager>,
    mut player_spawn_events: EventWriter<PlayerSpawnEvent>,
) {
    for event in connection_info_events.read() {
        let mut client = event.client.write();

        client.set_client_info(ClientInfo::new(event.login.clone()));

        log::info!(
            target: "network",
            "Connected ip:{} login:{} id:{}",
            client.get_client_ip(),
            client.get_client_info().unwrap().get_login(),
            client.get_client_id()
        );
        NetworkPlugin::send_resources(&client, &resources_manager);

        let default_world = "default".to_string();
        if worlds_manager.has_world_with_slug(&default_world) {
            let position = Position::new(0.0, 30.0, 0.0);
            let rotation = Rotation::new(0.0, 0.0);

            let mut world_manager = worlds_manager.get_world_manager_mut(&default_world).unwrap();
            let world_entity = world_manager.spawn_player(event.client.clone(), position, rotation);
            client.set_world_entity(Some(world_entity.clone()));

            client.network_send_teleport(&position, &rotation);

            if world_manager
                .get_chunks_map()
                .is_chunk_loaded(&position.get_chunk_position())
            {
                player_spawn_events.send(PlayerSpawnEvent::new(world_entity.clone()));
            }
        }
    }
}
