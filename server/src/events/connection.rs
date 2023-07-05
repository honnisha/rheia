use bevy_ecs::prelude::EventReader;
use bevy_ecs::system::{Res, ResMut};
use log::info;

use crate::network::player_container::Players;
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
            worlds_manager.spawn(&mut player, &default_teleport, 0_f32, 0_f32, 0_f32)
        }
    }
}
