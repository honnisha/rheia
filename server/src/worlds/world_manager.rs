use bevy_ecs::prelude::Entity;
use bevy_ecs::world::World;

use crate::entities::entity::{Indentifier, NetworkComponent, Position};

use crate::worlds::chunks::chunks_map::ChunkMap;

use super::chunks::chunks_map::ChunkPosition;

pub struct WorldManager {
    slug: String,
    world: World,
    chunks: ChunkMap,
}

impl WorldManager {
    pub fn new(slug: String) -> Self {
        WorldManager {
            slug: slug,
            world: World::new(),
            chunks: ChunkMap::new(),
        }
    }

    pub fn get_slug(&self) -> &String {
        &self.slug
    }

    pub fn spawn_player(&mut self, client_id: u64, x: f32, y: f32, z: f32) {
        self.world.spawn((
            Indentifier::default(),
            Position::new(x, y, z),
            NetworkComponent::new(client_id),
        ));
    }

    pub fn despawn_player(&mut self, client_id: &u64) {
        let mut query = self.world.query::<(Entity, &NetworkComponent)>();

        let mut obj_for_destroy = None;
        for (entity, network_component) in query.iter_mut(&mut self.world) {
            if network_component.client_id == *client_id {
                obj_for_destroy = Some(entity.clone());
                continue;
            }
        }

        if let Some(o) = obj_for_destroy {
            self.world.despawn(o);
        }
    }

    pub fn load_chunk(&mut self, chunk_position: ChunkPosition) {
    }

    pub fn unload_chunk(&mut self, chunk_position: ChunkPosition) {
    }
}
