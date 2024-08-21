use ahash::AHashMap;
use godot::prelude::*;

use super::entity::Entity;

#[derive(GodotClass)]
#[class(no_init, base=Node3D)]
pub struct EntitiesManager {
    pub base: Base<Node3D>,

    entities: AHashMap<u64, Gd<Entity>>,
}

impl EntitiesManager {
    pub fn create(base: Base<Node3D>) -> Self {
        Self {
            base,
            entities: Default::default(),
        }
    }
}
