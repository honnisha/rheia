use bevy_ecs::prelude::EventReader;
use bevy_ecs::system::{Res, ResMut};
use log::info;

use crate::entities::entity::Position;
use crate::network::player_container::Players;
use crate::network::server::NetworkContainer;
use crate::worlds::worlds_manager::WorldsManager;
use crate::{client_resources::resources_manager::ResourceManager, network::server::NetworkPlugin};

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
    players: Res<Players>,
    mut worlds_manager: ResMut<WorldsManager>,
) {
    for event in connection_events.iter() {
        let mut player = players.get_mut(&event.client_id);
        info!("Connected login \"{}\"", player.get_login());
        NetworkPlugin::send_resources(&event.client_id, &resources_manager);

        let default_teleport = "default".to_string();
        if worlds_manager.has_world_with_slug(&default_teleport) {
            let position = Position::new(0.0, 60.0, 0.0);
            worlds_manager.spawn_player(&mut player, &default_teleport, position.clone());
            network_container.teleport_player(&event.client_id, &default_teleport, &position);
            worlds_manager.send_loaded_chunks(&network_container, &*player);
        }
    }
}
