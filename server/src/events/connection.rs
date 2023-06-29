use bevy_ecs::system::Res;
use bevy_ecs::{prelude::EventReader, system::ResMut};
use bevy_renet::renet::RenetServer;
use log::info;

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
    mut server: ResMut<RenetServer>,
    mut connection_events: EventReader<PlayerConnectionEvent>,
    resources_manager: Res<ResourceManager>,
) {
    for event in connection_events.iter() {
        info!("Connected login \"{}\"", event.player_network.get_login());
        NetworkPlugin::send_resources(event.player_network.clone(), &resources_manager, &mut server);
    }
}
