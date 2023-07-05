use bevy_ecs::system::Res;
use bevy_ecs::{prelude::EventReader};
use log::info;

use crate::worlds::worlds_manager::WorldsManager;
use crate::{
    client_resources::resources_manager::ResourceManager,
    network::{player_network::PlayerNetwork, server::NetworkPlugin},
};

pub struct PlayerConnectionEvent {
    player_network: Box<PlayerNetwork>,
}

impl PlayerConnectionEvent {
    pub fn new(player_network: Box<PlayerNetwork>) -> Self {
        Self { player_network }
    }
}

pub fn on_connection(
    mut connection_events: EventReader<PlayerConnectionEvent>,
    resources_manager: Res<ResourceManager>,
    worlds_manager: Res<WorldsManager>,
) {
    for event in connection_events.iter() {
        info!("Connected login \"{}\"", event.player_network.get_login());
        NetworkPlugin::send_resources(event.player_network.clone(), &resources_manager);

        let default_teleport = "default".to_string();
        if worlds_manager.has_world_with_slug(&default_teleport) {
            let mut player_network = event.player_network.clone();
            player_network.teleport(&default_teleport, 0_f32, 0_f32, 0_f32);
        }
    }
}
