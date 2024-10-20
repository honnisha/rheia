use crate::entities::entity::{Position, Rotation};
use bevy::prelude::{Commands, Event, Res};
use bevy_ecs::prelude::EventReader;

use crate::{
    network::clients_container::ClientCell,
    worlds::{bevy_commands::SpawnPlayer, worlds_manager::WorldsManager},
};

#[derive(Event)]
pub struct PlayerSettingsLoadedEvent {
    client: ClientCell,
}

impl PlayerSettingsLoadedEvent {
    pub fn new(client: ClientCell) -> Self {
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

        commands.add(SpawnPlayer::create(
            default_world,
            event.client.clone(),
            Position::new(0.0, 30.0, 0.0),
            Rotation::new(0.0, 0.0),
        ));
    }
}
