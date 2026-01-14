use ahash::AHashMap;
use bevy::prelude::Entity;
use common::{
    chunks::{
        block_position::{BlockPosition, BlockPositionTrait},
        chunk_data::BlockDataInfo,
        chunk_position::ChunkPosition,
    }, utils::{spiral_iterator::SpiralIterator, vec_remove_item}, world_generator::{
        default::{WorldGenerator, WorldGeneratorSettings},
        traits::IWorldGenerator,
    }, worlds_storage::taits::IWorldStorage, WorldStorageManager, VERTICAL_SECTIONS
};
use parking_lot::{Mutex, RwLock, RwLockReadGuard};
use std::{sync::Arc, time::Duration};

use crate::{
    CHUNKS_DESPAWN_TIMER,
    network::runtime_plugin::RuntimePlugin,
    worlds::{chunks::chunk_column::load_chunk, world_manager::ChunkChanged},
};

use super::{chunk_column::ChunkColumn, chunks_load_state::ChunksLoadState};

pub type MapChunksType = AHashMap<ChunkPosition, Arc<RwLock<ChunkColumn>>>;

pub type ChunkSectionType<'a> = RwLockReadGuard<'a, ChunkColumn>;

pub type StorageLock = Arc<Mutex<WorldStorageManager>>;

/// Container of 2d ChunkColumn's.
/// This container manages vision of the chunks
/// and responsible for load/unload chunks
pub struct ChunkMap {
    // Hash map with chunk columns
    chunks: MapChunksType,

    chunks_load_state: ChunksLoadState,

    // A channel for tracking successfully uploaded chunks.
    loaded_chunks: (flume::Sender<ChunkPosition>, flume::Receiver<ChunkPosition>),

    world_generator: Arc<RwLock<WorldGenerator>>,

    storage: StorageLock,
}

impl ChunkMap {
    pub fn new(seed: u64, world_settings: WorldGeneratorSettings, storage: WorldStorageManager) -> Self {
        Self {
            chunks: Default::default(),
            chunks_load_state: Default::default(),
            loaded_chunks: flume::unbounded(),

            world_generator: Arc::new(RwLock::new(WorldGenerator::create(Some(seed), world_settings).unwrap())),
            storage: Arc::new(Mutex::new(storage)),
        }
    }

    pub fn drain_loaded_chunks(&self) -> flume::Drain<'_, ChunkPosition> {
        self.loaded_chunks.1.drain()
    }

    pub fn count(&self) -> usize {
        self.chunks.len()
    }

    pub fn get_chunks(&self) -> &MapChunksType {
        &self.chunks
    }

    pub fn is_chunk_loaded(&self, chunk_position: &ChunkPosition) -> bool {
        match self.chunks.get(&chunk_position) {
            Some(l) => l.read().is_loaded(),
            None => false,
        }
    }

    pub fn get_chunk_column(&self, chunk_position: &ChunkPosition) -> Option<ChunkSectionType> {
        match self.chunks.get(chunk_position) {
            Some(c) => Some(c.read()),
            None => None,
        }
    }

    /// Get all chunks watching by the player
    pub fn get_watching_chunks(&self, entity: &Entity) -> Option<&Vec<ChunkPosition>> {
        self.chunks_load_state.get_watching_chunks(entity)
    }

    /// Get Entities of all players watching the chunk
    ///
    /// All of them contains ClientNetwork
    pub fn get_chunk_watchers(&self, chunk_position: &ChunkPosition) -> Option<&Vec<Entity>> {
        self.chunks_load_state.get_chunk_watchers(&chunk_position)
    }

    /// Create player in the world
    pub fn start_chunks_render(&mut self, entity: Entity, to: &ChunkPosition, chunks_distance: u16) {
        let iter = SpiralIterator::new(to.x as i64, to.z as i64, chunks_distance as i64);
        for (x, z) in iter {
            let chunk_pos = ChunkPosition::new(x, z);
            self.chunks_load_state.insert_ticket(chunk_pos, entity.clone());

            // Update despawn timer
            if let Some(chunk_column) = self.chunks.get_mut(&chunk_pos) {
                chunk_column.read().set_despawn_timer(Duration::ZERO);
            }
        }
    }

    /*
        /// Gets the vector of old and new chunks when transitioning between chunks
        pub fn get_chunks_transition(
        from: &ChunkPosition,
        to: &ChunkPosition,
        chunks_distance: u16,
    ) -> (Vec<ChunkPosition>, Vec<ChunkPosition>) {
        let iter = ManhattanIterator::new(from.x as i32, from.z as i32, chunks_distance as i32);
        let mut old: Vec<ChunkPosition> = iter.map(|pos| ChunkPosition::new(pos.0 as i64, pos.1 as i64)).collect();

        let mut new: Vec<ChunkPosition> = Default::default();
        let iter = ManhattanIterator::new(to.x as i32, to.z as i32, chunks_distance as i32);
        for (x, z) in iter {
        let chunk = ChunkPosition::new(x as i64, z as i64);
        if !old.contains(&chunk) {
        new.push(chunk);
    }
    }

        old.retain(|&chunk| !new.contains(&chunk));
        (old, new)
    }
         */

    /// Trigered when player is move between chunks
    /// for updating chunks vision
    /// to unload unused chunks
    ///
    /// Returns unwatchd chunks
    pub fn update_chunks_render(
        &mut self,
        entity: Entity,
        from: &ChunkPosition,
        to: &ChunkPosition,
        chunks_distance: u16,
    ) -> ChunkChanged {
        if from == to {
            panic!("update_chunks_render from and to must be different chunks positions");
        }

        let mut old = self.chunks_load_state.get_watching_chunks(&entity).unwrap().clone();
        let mut new: Vec<ChunkPosition> = Default::default();

        // Add new tickets
        let iter = SpiralIterator::new(to.x, to.z, chunks_distance as i64);
        for (x, z) in iter {
            let chunk = ChunkPosition::new(x as i64, z as i64);

            // If its new chunk
            if !old.contains(&chunk) {
                // Start keeping this chunk
                self.chunks_load_state.insert_ticket(chunk, entity.clone());
                new.push(chunk.clone());

                // Update despawn timer
                if let Some(chunk_column) = self.chunks.get_mut(&chunk) {
                    chunk_column.read().set_despawn_timer(Duration::ZERO);
                }
            } else {
                // Remove chunk outside of side of view
                vec_remove_item(&mut old, &chunk);
            }
        }

        for chunk in old.iter() {
            // Stop keeping this chunk
            self.chunks_load_state.remove_ticket(&chunk, &entity);
        }

        ChunkChanged {
            old_chunk: from.clone(),
            new_chunk: to.clone(),
            abandoned_chunks: old,
            new_chunks: new,
        }
    }

    /// Player stop watch the world (despawn or move to another world)
    pub fn stop_chunks_render(&mut self, entity: Entity) {
        self.chunks_load_state.remove_all_entity_tickets(&entity);
    }

    /// Update chunks: load or despawn
    pub fn update_chunks(&mut self, delta: Duration, world_slug: &String) {
        // Update chunks despawn timer
        // Increase ONLY of noone looking at the chunk
        for (&chunk, chunk_column) in self.chunks.iter_mut() {
            if self.chunks_load_state.num_tickets(&chunk) == 0 {
                chunk_column.read().increase_despawn_timer(delta);
            }
        }

        // Despawn chunks waiting for despawn
        self.chunks.retain(|&chunk, chunk_column| {
            let chunk_column = chunk_column.read();
            let for_despawn = chunk_column.is_for_despawn(CHUNKS_DESPAWN_TIMER);
            if for_despawn {
                log::trace!(target: "chunks", "Chunk {} despawned", chunk);

                let save_chunk_data = self
                    .storage
                    .lock()
                    .save_chunk_data(chunk_column.get_chunk_position(), &chunk_column.sections);
                if let Err(e) = save_chunk_data {
                    log::error!(target: "worlds", "&cChunk save error!");
                    log::error!(target: "worlds", "Error: {}", e);
                    RuntimePlugin::stop();
                    panic!();
                }
            }
            !for_despawn
        });

        // Send to load new chunks
        for (chunk, players) in self.chunks_load_state.by_chunk.iter() {
            if players.len() == 0 {
                continue;
            }

            if !self.chunks.contains_key(&chunk) {
                let chunk_column = Arc::new(RwLock::new(ChunkColumn::new(chunk.clone(), world_slug.clone())));

                log::trace!(target: "chunks", "Send chunk {} to load", chunk);
                load_chunk(
                    self.world_generator.clone(),
                    self.storage.clone(),
                    chunk_column.clone(),
                    self.loaded_chunks.0.clone(),
                );
                self.chunks.insert(chunk.clone(), chunk_column);
            }
        }
    }

    pub fn edit_block(&self, position: BlockPosition, new_block_info: Option<BlockDataInfo>) -> Result<(), String> {
        let Some(chunk_column) = self.chunks.get(&position.get_chunk_position()) else {
            return Err(format!(
                "edit_block chunk {} is not found",
                position.get_chunk_position()
            ));
        };

        let (section, block_position) = position.get_block_position();
        if section > VERTICAL_SECTIONS as u32 {
            return Err(format!("edit_block section {} is more than", VERTICAL_SECTIONS));
        }
        chunk_column
            .write()
            .change_block(section, &block_position, new_block_info);
        return Ok(());
    }

    pub fn save(&mut self) -> Result<(), String> {
        for (_chunk_position, chunk_column) in self.chunks.iter() {
            let chunk_column = chunk_column.read();
            let save_chunk_data = self
                .storage
                .lock()
                .save_chunk_data(chunk_column.get_chunk_position(), &chunk_column.sections);
            if let Err(e) = save_chunk_data {
                return Err(e);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;
    use common::{
        WorldStorageManager,
        world_generator::default::WorldGeneratorSettings,
        worlds_storage::taits::{IWorldStorage, WorldStorageSettings},
    };
    use std::time::Duration;

    use crate::CHUNKS_DESPAWN_TIMER;

    use super::{ChunkMap, ChunkPosition};

    #[test]
    fn test_tickets_spawn_despawn() {
        let storage = WorldStorageManager::create("test".to_string(), 1, &WorldStorageSettings::default()).unwrap();
        let mut chunk_map = ChunkMap::new(1, WorldGeneratorSettings::default(), storage);
        let entity = Entity::from_raw(0);
        let chunks_distance = 2_u16;

        // Spawn
        let pos = ChunkPosition::new(0, 0);
        chunk_map.start_chunks_render(entity, &pos, chunks_distance);
        let chunks = chunk_map.chunks_load_state.get_watching_chunks(&entity).unwrap();
        assert_eq!(chunks.len(), 5);
        assert_eq!(chunks.contains(&ChunkPosition::new(0, 0)), true);
        assert_eq!(chunks.contains(&ChunkPosition::new(0, 1)), true);
        assert_eq!(chunks.contains(&ChunkPosition::new(0, -1)), true);
        assert_eq!(chunks.contains(&ChunkPosition::new(1, 0)), true);
        assert_eq!(chunks.contains(&ChunkPosition::new(-1, 0)), true);
        assert_eq!(chunk_map.chunks_load_state.num_tickets(&pos), 1);

        // Move
        let new_pos = ChunkPosition::new(1, 0);
        let change = chunk_map.update_chunks_render(entity, &pos, &new_pos, chunks_distance);
        let chunks = chunk_map.chunks_load_state.get_watching_chunks(&entity).unwrap();
        assert_eq!(chunks.len(), 5);
        assert_eq!(chunks.contains(&ChunkPosition::new(1, 0)), true);
        assert_eq!(chunks.contains(&ChunkPosition::new(1, 1)), true);
        assert_eq!(chunks.contains(&ChunkPosition::new(1, -1)), true);
        assert_eq!(chunks.contains(&ChunkPosition::new(2, 0)), true);
        assert_eq!(chunks.contains(&ChunkPosition::new(0, 0)), true);
        assert_eq!(chunk_map.chunks_load_state.num_tickets(&new_pos), 1);

        assert_eq!(change.abandoned_chunks.len(), 3);
        assert_eq!(change.abandoned_chunks.contains(&ChunkPosition::new(-1, 0)), true);
        assert_eq!(change.abandoned_chunks.contains(&ChunkPosition::new(0, 1)), true);
        assert_eq!(change.abandoned_chunks.contains(&ChunkPosition::new(0, -1)), true);

        // despawn
        chunk_map.stop_chunks_render(entity);
        assert_eq!(chunk_map.chunks_load_state.get_watching_chunks(&entity).is_none(), true);
        assert_eq!(chunk_map.chunks_load_state.num_tickets(&new_pos), 0);
    }

    #[test]
    fn test_update_chunks() {
        let storage = WorldStorageManager::create("test".to_string(), 1, &WorldStorageSettings::default()).unwrap();
        let mut chunk_map = ChunkMap::new(1, WorldGeneratorSettings::default(), storage);
        let world_slug = "default".to_string();
        let entity = Entity::from_raw(0);
        let pos = ChunkPosition::new(0, 0);

        chunk_map.chunks_load_state.insert_ticket(pos.clone(), entity.clone());
        chunk_map.update_chunks(Duration::from_secs(1), &world_slug);
        assert_eq!(chunk_map.chunks.len(), 1, "One chunk must be created");

        chunk_map
            .get_chunk_column(&pos)
            .unwrap()
            .set_despawn_timer(CHUNKS_DESPAWN_TIMER);

        chunk_map.chunks_load_state.remove_ticket(&pos, &entity);
        chunk_map.update_chunks(Duration::from_secs(1), &world_slug);
        assert_eq!(
            chunk_map.chunks.len(),
            0,
            "Because despawn_timer is fill - chunk must be unloaded"
        );
    }
}
