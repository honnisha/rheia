use bevy::prelude::Component;
use network::{entities::EntityNetworkComponent, messages::NetworkEntitySkin};

use super::traits::IEntityNetworkComponent;

#[derive(Component, Clone)]
pub struct EntitySkinComponent {
    skin: NetworkEntitySkin,
}

impl EntitySkinComponent {
    pub fn create(skin: NetworkEntitySkin) -> Self {
        Self { skin }
    }
}

impl IEntityNetworkComponent for EntitySkinComponent {
    fn to_network(&self) -> EntityNetworkComponent {
        EntityNetworkComponent::Skin(Some(self.skin.clone()))
    }

    fn get_empty() -> EntityNetworkComponent {
        EntityNetworkComponent::Skin(None)
    }
}
