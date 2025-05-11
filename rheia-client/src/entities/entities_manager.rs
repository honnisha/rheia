use ahash::AHashMap;
use common::chunks::rotation::Rotation;
use godot::prelude::*;
use network::{
    entities::EntityNetworkComponent,
    messages::{NetworkEntitySkin, NetworkEntityTag},
};

use super::entity::Entity;

#[derive(GodotClass)]
#[class(no_init, base=Node3D)]
pub struct EntitiesManager {
    pub base: Base<Node3D>,

    entities: AHashMap<u32, Gd<Entity>>,
}

impl EntitiesManager {
    pub fn create(base: Base<Node3D>) -> Self {
        Self {
            base,
            entities: Default::default(),
        }
    }

    pub fn _get(&self, entity_id: u32) -> Option<&Gd<Entity>> {
        self.entities.get(&entity_id)
    }

    pub fn create_entity(
        &mut self,
        id: u32,
        position: Vector3,
        rotation: Rotation,
        components: Vec<EntityNetworkComponent>,
    ) {
        if self.entities.contains_key(&id) {
            log::error!(target: "entities", "Tried to spawn existing entity id:{}", id);
            return;
        }

        let mut entity = Gd::<Entity>::from_init_fn(|base| Entity::create(base, components));
        self.base_mut().add_child(&entity);

        self.entities.insert(id, entity.clone());

        let mut e = entity.bind_mut();
        e.change_position(position);
        e.rotate(rotation);

        log::info!(target: "entities", "SPAWN id:{}", id);
    }

    pub fn update_entity_tag(&mut self, id: u32, tag: Option<NetworkEntityTag>) {
        let Some(entity) = self.entities.get_mut(&id) else {
            log::error!(target: "entities", "Tried to change tag for non-existent entity id:{}", id);
            return;
        };

        let mut e = entity.bind_mut();
        e.change_tag(tag);
    }

    pub fn update_entity_skin(&mut self, id: u32, skin: NetworkEntitySkin) {
        let Some(entity) = self.entities.get_mut(&id) else {
            log::error!(target: "entities", "Tried to change skin for non-existent entity id:{}", id);
            return;
        };

        let mut e = entity.bind_mut();
        e.change_skin(skin);
    }

    pub fn move_entity(&mut self, id: u32, position: Vector3, rotation: Rotation) {
        let Some(entity) = self.entities.get_mut(&id) else {
            log::error!(target:"entities", "Tried to move non-existent entity id:{}", id);
            return;
        };

        let mut e = entity.bind_mut();
        e.change_position(position);
        e.rotate(rotation);
    }

    pub fn despawn(&mut self, ids: Vec<u32>) {
        for id in ids.iter() {
            let Some(mut e) = self.entities.remove(id) else {
                log::error!(target:"entities", "Tried to despawn non-exitent entity id:{}", id);
                continue;
            };
            log::info!(target: "entities", "despawn id: {}", id);
            e.bind_mut().base_mut().queue_free();
        }
    }
}
