use bevy::prelude::Component;
use network::messages::{EntityNetworkComponent, EntityTag};

use super::traits::IEntityNetworkComponent;

#[derive(Component, Clone)]
pub struct EntityTagComponent(EntityTag);

impl EntityTagComponent {
    pub fn create(tag: EntityTag) -> Self {
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
