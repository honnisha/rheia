use super::ecs::Ecs;
use crate::CHUNKS_DISTANCE;
use crate::entities::EntityComponent;
use crate::entities::entity::{Position, Rotation};
use crate::network::client_network::WorldEntity;
use crate::worlds::chunks::chunks_map::ChunkMap;
use bevy_ecs::bundle::Bundle;
use common::WorldStorageManager;
use common::chunks::block_position::BlockPositionTrait;
use common::chunks::chunk_data::BlockIndexType;
use common::chunks::chunk_position::ChunkPosition;
use common::world_generator::default::WorldGeneratorSettings;
use common::worlds_storage::taits::{IWorldStorage, WorldStorageSettings};
use network::messages::ServerMessages;
use std::collections::BTreeMap;
use std::time::Duration;

pub struct ChunkChanged {
    pub old_chunk: ChunkPosition,
    pub new_chunk: ChunkPosition,
    pub abandoned_chunks: Vec<ChunkPosition>,
    pub new_chunks: Vec<ChunkPosition>,
}

pub struct WorldManager {
    slug: String,
    ecs: Ecs,
    chunks_map: ChunkMap,
}

impl WorldManager {
    pub fn new(
        slug: String,
        seed: u64,
        world_settings: WorldGeneratorSettings,
        world_storage_settings: &WorldStorageSettings,
        block_id_map: &BTreeMap<BlockIndexType, String>,
    ) -> Result<Self, String> {
        let storage = match WorldStorageManager::create(slug.clone(), seed.clone(), world_storage_settings) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };
        if let Err(e) = WorldStorageManager::validate_block_id_map(slug.clone(), world_storage_settings, block_id_map) {
            return Err(e);
        }
        Ok(WorldManager {
            slug: slug,
            ecs: Ecs::new(),
            chunks_map: ChunkMap::new(seed, world_settings, storage),
        })
    }

    pub fn get_ecs(&self) -> &Ecs {
        &self.ecs
    }

    pub fn get_ecs_mut(&mut self) -> &mut Ecs {
        &mut self.ecs
    }

    pub fn get_chunks_map(&self) -> &ChunkMap {
        &self.chunks_map
    }

    pub fn get_chunks_map_mut(&mut self) -> &mut ChunkMap {
        &mut self.chunks_map
    }

    pub fn get_slug(&self) -> &String {
        &self.slug
    }

    pub fn get_chunks_count(&self) -> usize {
        self.get_chunks_map().count()
    }

    pub fn spawn_player<B: Bundle>(
        &mut self,
        position: Position,
        bundle: B,
        components: Vec<EntityComponent>,
    ) -> WorldEntity {
        let entity = self.get_ecs_mut().spawn(bundle, position.get_chunk_position());

        let mut entity_ecs = self.get_ecs_mut().entity_mut(entity);
        if components.len() > 0 {
            for component in components {
                match component {
                    EntityComponent::Tag(c) => {
                        if let Some(c) = c {
                            entity_ecs.insert(c);
                        }
                    }
                    EntityComponent::Skin(c) => {
                        if let Some(c) = c {
                            entity_ecs.insert(c);
                        }
                    }
                }
            }
        }

        self.get_chunks_map_mut()
            .start_chunks_render(entity, &position.get_chunk_position(), CHUNKS_DISTANCE);

        WorldEntity::new(self.slug.clone(), entity)
    }

    /// Records the player's movement and updates his position in ECS.
    ///
    /// Returns boolean if player changed his chunk and his despawned chunks
    pub fn player_move(
        &mut self,
        world_entity: &WorldEntity,
        position: Position,
        rotation: Rotation,
    ) -> Option<ChunkChanged> {
        let mut changed_chunks: Option<ChunkChanged> = None;

        let mut player_entity = self.ecs.entity_mut(world_entity.get_entity());
        let mut old_position = player_entity.get_mut::<Position>().unwrap();

        let old_chunk = old_position.get_chunk_position();
        let new_chunk = position.get_chunk_position();
        let chunk_changed = old_chunk != new_chunk;
        if chunk_changed {
            let chunks = self.chunks_map.update_chunks_render(
                world_entity.get_entity(),
                &old_chunk,
                &new_chunk,
                CHUNKS_DISTANCE,
            );
            changed_chunks = Some(chunks);
        }
        *old_position = position;
        let mut old_rotation = player_entity.get_mut::<Rotation>().unwrap();
        *old_rotation = rotation;

        if chunk_changed {
            self.ecs
                .entity_moved_chunk(&world_entity.get_entity(), &old_chunk, &new_chunk);
        }
        changed_chunks
    }

    pub fn save(&mut self) -> Result<(), String> {
        self.chunks_map.save()?;
        log::info!(target: "worlds", "World &a\"{}\"&r saved", self.slug);
        Ok(())
    }

    pub fn despawn_player(&mut self, world_entity: &WorldEntity) {
        self.get_chunks_map_mut().stop_chunks_render(world_entity.get_entity());

        let player_entity = self.ecs.get_entity(world_entity.get_entity()).unwrap();
        let chunk_position = match player_entity.get::<Position>() {
            Some(p) => Some(p.get_chunk_position()),
            None => None,
        };

        self.get_ecs_mut().despawn(world_entity.get_entity(), chunk_position);
    }

    /// Proxy for sending update_chunks
    pub fn update_chunks(&mut self, delta: Duration) {
        let world_slug = self.get_slug().clone();
        self.chunks_map.update_chunks(delta, &world_slug);
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
