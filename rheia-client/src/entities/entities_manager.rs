use ahash::AHashMap;
use common::network::messages::Rotation;
use godot::prelude::*;

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

    pub fn create_entity(&mut self, id: u32, position: Vector3, rotation: Rotation) {
        if self.entities.contains_key(&id) {
            log::error!(target:"entities", "Tried to spawn existing entity id:{}", id);
            return;
        }

        let mut entity = Gd::<Entity>::from_init_fn(|base| Entity::create(base));
        self.base_mut().add_child(entity.clone().upcast());

        entity.set_position(position);
        entity.bind_mut().rotate(rotation);

        self.entities.insert(id, entity);
        log::info!(target:"entities", "SPAWN id:{}", id);
    }

    pub fn move_entity(&mut self, id: u32, position: Vector3, rotation: Rotation) {
        let Some(e) = self.entities.get_mut(&id) else {
            log::error!(target:"entities", "Tried to move non existent entity id:{}", id);
            return;
        };

        e.set_position(position);

        let movement = position - e.get_position();
        let mut gd = e.bind_mut();
        gd.handle_movement(movement);
        gd.rotate(rotation);
    }

    pub fn despawn(&mut self, ids: Vec<u32>) {
        for id in ids.iter() {
            let Some(mut e) = self.entities.remove(id) else {
                log::error!(target:"entities", "Tried to despawn non exitent entity id:{}", id);
                continue;
            };
            log::info!(target:"entities", "despawn id: {}", id);
            e.bind_mut().base_mut().queue_free();
        }
    }
}
