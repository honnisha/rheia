use ahash::{AHashMap, HashSet};
use common::{
    blocks::block_info::BlockInfo,
    chunks::{
        block_position::{BlockPosition, BlockPositionTrait},
        chunk_position::ChunkPosition,
    },
    VERTICAL_SECTIONS,
};
use godot::prelude::*;
use network::messages::SectionsData;
use parking_lot::RwLock;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use crate::world::{
    physics::PhysicsProxy,
    worlds_manager::{BlockStorageRef, TextureMapperRef},
};

use super::{
    chunk_column::{ChunkColumn, ColumnDataLockType},
    chunk_generator::generate_chunk,
    near_chunk_data::NearChunksData,
};
use flume::{unbounded, Receiver, Sender};

pub type ChunkLock = Arc<RwLock<ChunkColumn>>;
pub type ChunksType = AHashMap<ChunkPosition, ChunkLock>;

/// Container of all chunk sections
#[derive(GodotClass)]
#[class(no_init, base=Node)]
pub struct ChunkMap {
    pub(crate) base: Base<Node>,

    // Hash map with chunk columns
    chunks: ChunksType,

    sended_chunks: Rc<RefCell<HashSet<ChunkPosition>>>,
    chunks_to_spawn: (Sender<ChunkLock>, Receiver<ChunkLock>),

    chunks_to_update: Rc<RefCell<HashSet<(ChunkPosition, usize)>>>,
}

impl ChunkMap {
    pub fn create(base: Base<Node>) -> Self {
        Self {
            base,
            chunks: Default::default(),
            sended_chunks: Default::default(),
            chunks_to_spawn: unbounded(),
            chunks_to_update: Default::default(),
        }
    }

    pub fn get_chunks_count(&self) -> usize {
        self.chunks.len()
    }

    pub fn get_chunk(&self, chunk_position: &ChunkPosition) -> Option<ChunkLock> {
        match self.chunks.get(chunk_position) {
            Some(c) => Some(c.clone()),
            None => None,
        }
    }

    pub fn _get_chunk_column_data(&self, chunk_position: &ChunkPosition) -> Option<ColumnDataLockType> {
        match self.chunks.get(chunk_position) {
            Some(c) => Some(c.read().get_chunk_data().clone()),
            None => None,
        }
    }

    pub fn load_chunk(&mut self, chunk_position: ChunkPosition, sections: SectionsData) {
        if self.chunks.contains_key(&chunk_position) {
            log::error!(
                target: "chunk_map",
                "Network sended chunk to load, but it already exists: {}",
                chunk_position
            );
            return;
        }

        let chunk_column = ChunkColumn::create(chunk_position, sections);
        self.chunks
            .insert(chunk_position.clone(), Arc::new(RwLock::new(chunk_column)));
        self.sended_chunks.borrow_mut().insert(chunk_position);
    }

    pub fn unload_chunk(&mut self, chunk_position: ChunkPosition) {
        let mut unloaded = false;
        if let Some(chunk_column) = self.chunks.remove(&chunk_position) {
            chunk_column.write().free();
            unloaded = true;
        }
        if !unloaded {
            log::error!(target: "chunk_map", "Unload chunk not found: {}", chunk_position);
        }
    }

    /// Send new recieved chunks to load (render)
    pub fn send_chunks_to_load(&mut self, material_instance_id: InstanceId) {
        self.sended_chunks.borrow_mut().retain(|chunk_position| {
            let near_chunks_data = NearChunksData::new(&self.chunks, &chunk_position);

            // Load only if all chunks around are loaded
            if !near_chunks_data.is_full() {
                return true;
            }

            let chunk_column = self
                .get_chunk(&chunk_position)
                .expect("chunk from sended_chunks is not found");
            generate_chunk(
                chunk_column.clone(),
                near_chunks_data,
                self.chunks_to_spawn.0.clone(),
                material_instance_id,
            );
            return false;
        });
    }

    /// Retrieving loaded chunks to add them to the root node
    pub fn spawn_loaded_chunks(&mut self, physics: &PhysicsProxy) {
        let mut base = self.base_mut().clone();
        for l in self.chunks_to_spawn.1.drain() {
            let mut chunk_column = l.write();

            let chunk_base = chunk_column.get_base();
            base.add_child(chunk_base.clone().upcast());
            chunk_column.spawn_loaded_chunk();
            chunk_column.update_geometry(physics);
        }
    }

    pub fn change_active_chunk(&mut self, active_chunk_position: &ChunkPosition) {
        for (chunk_position, chunk_lock) in self.chunks.iter_mut() {
            chunk_lock.write().set_active(chunk_position == active_chunk_position);
        }
    }

    pub fn edit_block(&self, position: BlockPosition, new_block_info: BlockInfo) {
        let Some(chunk_column) = self.chunks.get(&position.get_chunk_position()) else {
            return;
        };

        let (section, block_position) = position.get_block_position();
        if section > VERTICAL_SECTIONS as u32 {
            return;
        }
        chunk_column
            .write()
            .change_block_info(section, block_position, new_block_info);
        self.chunks_to_update
            .borrow_mut()
            .insert((position.get_chunk_position(), section as usize));
    }

    pub fn update_chunks(
        &self,
        physics: &PhysicsProxy,
        block_storage: &BlockStorageRef,
        texture_mapper: &TextureMapperRef,
    ) {
        self.chunks_to_update.borrow_mut().retain(|(chunk_position, y)| {
            let near_chunks_data = NearChunksData::new(&self.chunks, &chunk_position);

            // Load only if all chunks around are loaded
            if !near_chunks_data.is_full() {
                return true;
            }

            let chunk_column = self
                .get_chunk(&chunk_position)
                .expect("chunk from chunks_to_update is not found");
            let mut c = chunk_column.write();
            c.generate_section_geometry(&near_chunks_data, *y, &block_storage, &texture_mapper);
            c.update_geometry(physics);
            return false;
        });
    }
}
