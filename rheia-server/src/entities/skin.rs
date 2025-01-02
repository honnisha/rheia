use bevy::prelude::Component;
use network::messages::EntitySkin as NetworkEntitySkin;

#[derive(Component, Clone)]
pub struct EntitySkin {
    skin: NetworkEntitySkin,
}

impl EntitySkin {
    pub fn create(skin: NetworkEntitySkin) -> Self {
        Self { skin }
    }

    pub fn to_network(&self) -> &NetworkEntitySkin {
        &self.skin
    }
}
