use std::sync::Arc;
use std::time::Duration;

use bevy_ecs::prelude::Entity;
use bevy_ecs::world::World;
use parking_lot::RwLock;

use crate::entities::entity::{Indentifier, NetworkComponent, Position};

use crate::worlds::chunks::chunks_map::ChunkMap;

use super::world_generator::WorldGenerator;

pub struct WorldManager {
    slug: String,
    world: World,
    chunks: ChunkMap,
    world_generator: Arc<RwLock<WorldGenerator>>,
}

impl WorldManager {
    pub fn new(slug: String, seed: u64) -> Self {
        WorldManager {
            slug: slug,
            world: World::new(),
            chunks: ChunkMap::new(),
            world_generator: Arc::new(RwLock::new(WorldGenerator::new(seed))),
        }
    }

    pub fn get_slug(&self) -> &String {
        &self.slug
    }

    pub fn spawn_player(&mut self, client_id: u64, x: f32, y: f32, z: f32) {
        let new_position = Position::new(x, y, z);
        self.chunks
            .update_chunks_render(&client_id, None, Some(&new_position.get_chunk_position()));
        self.world
            .spawn((Indentifier::default(), new_position, NetworkComponent::new(client_id)));
    }

    pub fn despawn_player(&mut self, client_id: &u64) {
        let mut query = self.world.query::<(Entity, &NetworkComponent, &Position)>();

        let mut obj_for_destroy = None;
        for (entity, network_component, position) in query.iter_mut(&mut self.world) {
            if network_component.client_id == *client_id {
                self.chunks
                    .update_chunks_render(client_id, Some(&position.get_chunk_position()), None);
                obj_for_destroy = Some(entity.clone());
                continue;
            }
        }

        if let Some(o) = obj_for_destroy {
            self.world.despawn(o);
        }
    }

    pub fn update_chunks(&mut self, delta: Duration) {
        let world_slug = self.get_slug().clone();
        self.chunks.update_chunks(delta, &world_slug, self.world_generator.clone());
    }
}
