use entity_tag::EntityTag;
use serde::{Deserialize, Serialize};

pub mod entity_tag;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EntitySkin {
    Generic,
    Fixed(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EntityNetworkComponent {
    Tag(Option<EntityTag>),
    Skin(Option<EntitySkin>),
}
