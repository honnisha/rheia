use entity_tag::EntityTagComponent;
use network::entities::EntityNetworkComponent;
use skin::EntitySkinComponent;
use strum_macros::EnumIter;
use traits::IEntityNetworkComponent;

pub mod commands;
pub mod entity;
pub mod entity_tag;
pub mod events;
pub mod skin;
pub mod traits;

#[derive(Clone, EnumIter)]
/// Needs to be added in `send_start_streaming_entity`
pub enum EntityComponent {
    Tag(Option<EntityTagComponent>),
    Skin(Option<EntitySkinComponent>),
}

impl EntityComponent {
    pub fn to_network(&self) -> EntityNetworkComponent {
        match self {
            EntityComponent::Tag(c) => match c {
                Some(c) => c.to_network(),
                None => EntityNetworkComponent::Skin(None),
            },
            EntityComponent::Skin(c) => match c {
                Some(c) => c.to_network(),
                None => EntityNetworkComponent::Skin(None),
            },
        }
    }
}
