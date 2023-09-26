use std::sync::Arc;
use std::time::Duration;

use bevy_ecs::world::World;
use common::chunks::block_position::BlockPositionTrait;
use common::chunks::chunk_position::ChunkPosition;
use common::network::messages::ServerMessages;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::entities::entity::{NetworkComponent, Position, Rotation};
use crate::CHUNKS_DISTANCE;

use crate::network::client_network::WorldEntity;
use crate::worlds::chunks::chunks_map::ChunkMap;

use super::world_generator::WorldGenerator;

pub struct WorldManager {
    slug: String,
    world: Arc<RwLock<World>>,
    chunks_map: Arc<RwLock<ChunkMap>>,
    world_generator: Arc<RwLock<WorldGenerator>>,
}

impl WorldManager {
    pub fn new(slug: String, seed: u64) -> Self {
        WorldManager {
            slug: slug,
            world: Arc::new(RwLock::new(World::new())),
            chunks_map: Arc::new(RwLock::new(ChunkMap::new())),
            world_generator: Arc::new(RwLock::new(WorldGenerator::new(seed))),
        }
    }

    pub fn get_ecs(&self) -> RwLockReadGuard<World> {
        self.world.read()
    }

    pub fn get_ecs_mut(&self) -> RwLockWriteGuard<World> {
        self.world.write()
    }

    pub fn get_chunks_map(&self) -> RwLockReadGuard<ChunkMap> {
        self.chunks_map.read()
    }

    pub fn get_chunks_map_mut(&self) -> RwLockWriteGuard<ChunkMap> {
        self.chunks_map.write()
    }

    pub fn get_slug(&self) -> &String {
        &self.slug
    }

    pub fn get_chunks_count(&self) -> usize {
        self.get_chunks_map().count()
    }

    pub fn spawn_player(&mut self, client_id: &u64, position: Position, rotation: Rotation) -> WorldEntity {
        let ecs = (position, rotation, NetworkComponent::new(client_id.clone()));

        let entity = {
            let mut world = self.get_ecs_mut();
            world.spawn(ecs).id()
        };

        self.get_chunks_map_mut()
            .start_chunks_render(entity, &position.get_chunk_position(), CHUNKS_DISTANCE);

        WorldEntity::new(self.slug.clone(), entity)
    }

    /// Returns boolean if player changed his chunk
    /// and his despawned chunks if so
    pub fn player_move(
        &self,
        world_entity: &WorldEntity,
        position: Position,
        rotation: Rotation,
    ) -> (bool, Vec<ChunkPosition>) {
        let mut abandoned_chunks: Vec<ChunkPosition> = Default::default();

        let mut w = self.get_ecs_mut();
        let mut player_entity = w.entity_mut(world_entity.get_entity());
        let mut old_position = player_entity.get_mut::<Position>().unwrap();

        let old_chunk = old_position.get_chunk_position();
        let new_chunk = position.get_chunk_position();
        let chunk_changed = old_chunk != new_chunk;
        if chunk_changed {
            abandoned_chunks = self.get_chunks_map_mut().update_chunks_render(
                world_entity.get_entity(),
                &old_chunk,
                &new_chunk,
                CHUNKS_DISTANCE,
            );
        }
        *old_position = position;
        let mut old_rotation = player_entity.get_mut::<Rotation>().unwrap();
        *old_rotation = rotation;

        (chunk_changed, abandoned_chunks)
    }

    pub fn despawn_player(&mut self, world_entity: &WorldEntity) {
        self.get_chunks_map_mut().stop_chunks_render(world_entity.get_entity());
        self.get_ecs_mut().despawn(world_entity.get_entity());
    }

    /// Proxy for sending update_chunks
    pub fn update_chunks(&mut self, delta: Duration) {
        let world_slug = self.get_slug().clone();
        self.get_chunks_map_mut()
            .update_chunks(delta, &world_slug, self.world_generator.clone());
    }

    pub fn get_network_chunk_bytes(&self, chunk_position: &ChunkPosition) -> Option<ServerMessages> {
        match self.get_chunks_map().get_chunk_column(&chunk_position) {
            Some(chunk_column) => {
                if !chunk_column.is_loaded() {
                    return None;
                }
                Some(chunk_column.build_network_format())
            }
            None => None,
        }
    }
}
