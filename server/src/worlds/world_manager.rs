use std::sync::Arc;
use std::time::Duration;

use bevy_ecs::prelude::Entity;
use bevy_ecs::world::{EntityRef, World};
use common::chunks::block_position::BlockPositionTrait;
use common::chunks::chunk_position::ChunkPosition;
use common::network::messages::ServerMessages;
use parking_lot::RwLock;

use crate::entities::entity::{NetworkComponent, Position, Rotation};
use crate::CHUNKS_DISTANCE;

use crate::network::client_network::WorldEntity;
use crate::worlds::chunks::chunks_map::ChunkMap;

use super::world_generator::WorldGenerator;

pub struct WorldManager {
    slug: String,
    world: World,
    pub(crate) chunks_map: ChunkMap,
    world_generator: Arc<RwLock<WorldGenerator>>,
}

impl WorldManager {
    pub fn new(slug: String, seed: u64) -> Self {
        WorldManager {
            slug: slug,
            world: World::new(),
            chunks_map: ChunkMap::new(),
            world_generator: Arc::new(RwLock::new(WorldGenerator::new(seed))),
        }
    }

    pub fn get_slug(&self) -> &String {
        &self.slug
    }

    pub fn get_chunks_count(&self) -> usize {
        self.chunks_map.count()
    }

    pub fn get_entity(&self, entity: &Entity) -> EntityRef {
        self.world.entity(*entity)
    }

    pub fn spawn_player(&mut self, client_id: &u64, position: Position, rotation: Rotation) -> WorldEntity {
        let ecs = (position, rotation, NetworkComponent::new(client_id.clone()));
        let entity = self.world.spawn(ecs);
        self.chunks_map
            .start_chunks_render(entity.id(), &position.get_chunk_position(), CHUNKS_DISTANCE);

        WorldEntity::new(self.slug.clone(), entity.id())
    }

    pub fn player_move(
        &mut self,
        world_entity: &WorldEntity,
        position: Position,
        rotation: Rotation,
    ) -> Vec<ChunkPosition> {
        let mut abandoned_chunks: Vec<ChunkPosition> = Default::default();

        let mut player_entity = self.world.entity_mut(world_entity.get_entity());
        let mut old_position = player_entity.get_mut::<Position>().unwrap();

        let old_chunk = old_position.get_chunk_position();
        let new_chunk = position.get_chunk_position();
        if old_chunk != new_chunk {
            abandoned_chunks = self.chunks_map.update_chunks_render(
                world_entity.get_entity(),
                &old_chunk,
                &new_chunk,
                CHUNKS_DISTANCE,
            );
        }
        *old_position = position;
        let mut old_rotation = player_entity.get_mut::<Rotation>().unwrap();
        *old_rotation = rotation;

        abandoned_chunks
    }

    pub fn despawn_player(&mut self, world_entity: &WorldEntity) {
        self.chunks_map.stop_chunks_render(world_entity.get_entity());
        self.world.despawn(world_entity.get_entity());
    }

    /// Proxy for sending update_chunks
    pub fn update_chunks(&mut self, delta: Duration) {
        let world_slug = self.get_slug().clone();
        self.chunks_map
            .update_chunks(delta, &world_slug, self.world_generator.clone());
    }

    pub fn get_network_chunk_bytes(&self, chunk_position: &ChunkPosition) -> Option<Vec<u8>> {
        match self.chunks_map.get_chunk_column(&chunk_position) {
            Some(chunk_column) => {
                if !chunk_column.is_loaded() {
                    return None;
                }
                let input = ServerMessages::ChunkSectionInfo {
                    sections: chunk_column.build_network_format(),
                    chunk_position: chunk_position.clone(),
                };
                Some(bincode::serialize(&input).unwrap())
            }
            None => None,
        }
    }
}
