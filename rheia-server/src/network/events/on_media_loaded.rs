use crate::entities::entity::{Position, Rotation};
use bevy::prelude::{Event, Res};
use bevy_ecs::prelude::EventReader;

use crate::{network::clients_container::ClientCell, worlds::worlds_manager::WorldsManager};

#[derive(Event)]
pub struct PlayerMediaLoadedEvent {
    client: ClientCell,
}

impl PlayerMediaLoadedEvent {
    pub fn new(client: ClientCell) -> Self {
        Self { client }
    }
}

pub fn on_media_loaded(mut events: EventReader<PlayerMediaLoadedEvent>, worlds_manager: Res<WorldsManager>) {
    for event in events.read() {
        let client = event.client.read();
    }
}
