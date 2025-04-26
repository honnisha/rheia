use crate::{
    entities::{
        entity::{Position, Rotation},
        skin::EntitySkin,
    },
    network::client_network::ClientNetwork,
    worlds::bevy_commands::UpdatePlayerSkin,
};
use bevy::prelude::{Commands, Event, Res};
use bevy_ecs::prelude::EventReader;
use network::messages::EntitySkin as NetworkEntitySkin;

use crate::worlds::{bevy_commands::SpawnPlayer, worlds_manager::WorldsManager};

#[derive(Event)]
pub struct PlayerSettingsLoadedEvent {
    client: ClientNetwork,
}

impl PlayerSettingsLoadedEvent {
    pub fn new(client: ClientNetwork) -> Self {
        Self { client }
    }
}

pub fn on_settings_loaded(
    mut commands: Commands,
    mut events: EventReader<PlayerSettingsLoadedEvent>,
    worlds_manager: Res<WorldsManager>,
) {
    for event in events.read() {
        let default_world = "default".to_string();
        if !worlds_manager.has_world_with_slug(&default_world) {
            panic!("default world is not found");
        };

        commands.queue(SpawnPlayer::create(
            default_world,
            event.client.clone(),
            Position::new(0.0, 30.0, 0.0),
            Rotation::new(0.0, 0.0),
        ));
        commands.queue(UpdatePlayerSkin::create(
            event.client.clone(),
            Some(EntitySkin::create(NetworkEntitySkin::Generic)),
        ));
    }
}
