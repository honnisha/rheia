use crate::{
    entities::{
        entity::{Position, Rotation},
        entity_tag::EntityTagComponent,
        skin::EntitySkinComponent,
        EntityComponent,
    },
    network::client_network::ClientNetwork,
};
use bevy::prelude::{Commands, Event, Res};
use bevy_ecs::prelude::EventReader;
use network::messages::{NetworkEntitySkin, NetworkEntityTag};

use crate::worlds::{commands::SpawnPlayer, worlds_manager::WorldsManager};

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

        let mut components: Vec<EntityComponent> = Default::default();

        let skin = EntitySkinComponent::create(NetworkEntitySkin::Generic);
        components.push(EntityComponent::Skin(Some(skin)));

        let client_info = event.client.get_client_info().unwrap();
        let tag = EntityTagComponent::create(NetworkEntityTag::create(client_info.get_login().clone(), 2.1, 32, 3));
        components.push(EntityComponent::Tag(Some(tag)));

        commands.queue(SpawnPlayer::create(
            default_world,
            event.client.clone(),
            Position::new(0.0, 80.0, 0.0),
            Rotation::new(0.0, 0.0),
            components,
        ));

        // let skin = EntitySkinComponent::create(NetworkEntitySkin::Generic);
        // commands.queue(UpdatePlayerComponent::create(
        //     event.client.clone(),
        //     EntityComponent::Skin(Some(skin)),
        // ));
    }
}
