use bevy::prelude::Component;
use network::messages::{EntityNetworkComponent, EntitySkin};

use super::traits::IEntityNetworkComponent;

#[derive(Component, Clone)]
pub struct EntitySkinComponent {
    skin: EntitySkin,
}

impl EntitySkinComponent {
    pub fn create(skin: EntitySkin) -> Self {
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
