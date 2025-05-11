use bevy::prelude::Component;
use network::{entities::EntityNetworkComponent, messages::NetworkEntityTag};

use super::traits::IEntityNetworkComponent;

#[derive(Component, Clone)]
pub struct EntityTagComponent(NetworkEntityTag);

impl EntityTagComponent {
    pub fn create(tag: NetworkEntityTag) -> Self {
        Self(tag)
    }
}

impl IEntityNetworkComponent for EntityTagComponent {
    fn to_network(&self) -> EntityNetworkComponent {
        EntityNetworkComponent::Tag(Some(self.0.clone()))
    }

    fn get_empty() -> EntityNetworkComponent {
        EntityNetworkComponent::Tag(None)
    }
}
