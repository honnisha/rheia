use std::sync::Arc;
use std::time::Duration;

use bevy_ecs::prelude::Entity;
use bevy_ecs::world::World;
use common::chunks::block_position::BlockPositionTrait;
use common::chunks::chunk_position::ChunkPosition;
use common::network::channels::server_reliable::ServerMessages;
use parking_lot::RwLock;

use crate::entities::entity::{Indentifier, NetworkComponent, Position};
use crate::CHUNKS_DISTANCE;

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

    pub fn spawn_player(&mut self, client_id: u64, position: Position) {
        self.chunks_map
            .update_chunks_render(&client_id, None, Some(&position.get_chunk_position()), CHUNKS_DISTANCE);
        self.world
            .spawn((Indentifier::default(), position, NetworkComponent::new(client_id)));
    }

    pub fn despawn_player(&mut self, client_id: &u64) {
        let mut query = self.world.query::<(Entity, &NetworkComponent, &Position)>();

        let mut obj_for_destroy = None;
        for (entity, network_component, position) in query.iter_mut(&mut self.world) {
            if network_component.client_id == *client_id {
                self.chunks_map.update_chunks_render(
                    client_id,
                    Some(&position.get_chunk_position()),
                    None,
                    CHUNKS_DISTANCE,
                );
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
