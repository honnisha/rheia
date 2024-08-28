use ahash::AHashMap;
use common::chunks::{chunk_position::ChunkPosition, utils::SectionsData};
use godot::{engine::Material, prelude::*};
use log::error;
use parking_lot::RwLock;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use crate::world::{physics::PhysicsProxy, worlds_manager::TextureMapperType};

use super::{
    chunk_column::{ChunkBase, ChunkColumn, ColumnDataLockType},
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

    texture_mapper: TextureMapperType,
    material: Gd<Material>,

    sended_chunks: Rc<RefCell<Vec<ChunkPosition>>>,
    loaded_chunks: (
        Sender<(ChunkPosition, InstanceId)>,
        Receiver<(ChunkPosition, InstanceId)>,
    ),
}

impl ChunkMap {
    pub fn create(base: Base<Node>, texture_mapper: TextureMapperType, material: Gd<Material>) -> Self {
        Self {
            base,
            chunks: Default::default(),
            texture_mapper,
            material,
            sended_chunks: Default::default(),
            loaded_chunks: unbounded(),
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

    pub fn get_chunk_column_data(&self, chunk_position: &ChunkPosition) -> Option<ColumnDataLockType> {
        match self.chunks.get(chunk_position) {
            Some(c) => Some(c.read().get_chunk_data().clone()),
            None => None,
        }
    }

    pub fn load_chunk(&mut self, chunk_position: ChunkPosition, sections: SectionsData) {
        if self.chunks.contains_key(&chunk_position) {
            error!(
                "Network sended chunk to load, but it already exists: {}",
                chunk_position
            );
            return;
        }

        let chunk_column = ChunkColumn::create(chunk_position, sections);
        self.chunks
            .insert(chunk_position.clone(), Arc::new(RwLock::new(chunk_column)));
        self.sended_chunks.borrow_mut().push(chunk_position);
    }

    pub fn unload_chunk(&mut self, chunk_position: ChunkPosition) {
        let mut unloaded = false;
        if let Some(chunk_column) = self.chunks.remove(&chunk_position) {
            chunk_column.write().free();
            unloaded = true;
        }
        if !unloaded {
            error!("Unload chunk not found: {}", chunk_position);
        }
    }

    /// Send new recieved chunks to load (render)
    pub fn send_chunks_to_load(&mut self, physics: &PhysicsProxy) {
        self.sended_chunks.borrow_mut().retain(|chunk_position| {
            let near_chunks_data = NearChunksData::new(&self.chunks, &chunk_position);

            // Load only if all chunks around are loaded
            if !near_chunks_data.is_full() {
                return true;
            }

            let chunk_column = self.get_chunk(&chunk_position).unwrap();
            generate_chunk(
                chunk_column.read().get_chunk_data().clone(),
                near_chunks_data,
                self.texture_mapper.clone(),
                self.material.instance_id(),
                chunk_position.clone(),
                physics.clone(),
                self.loaded_chunks.0.clone(),
            );
            chunk_column.read().set_sended();
            return false;
        });
    }

    /// Retrieving loaded chunks to add them to the root node
    pub fn spawn_loaded_chunks(&mut self) {
        let mut base = self.base_mut().clone();
        for (chunk_position, instance_id) in self.loaded_chunks.1.drain() {
            let l = self.get_chunk(&chunk_position).unwrap();
            let mut chunk_column = l.write();

            let chunk_base = Gd::<ChunkBase>::from_instance_id(instance_id);
            base.add_child(chunk_base.clone().upcast());
            chunk_column.spawn_loaded_chunk(chunk_base);
        }
    }

    pub fn change_active_chunk(&mut self, active_chunk_position: &ChunkPosition) {
        for (chunk_position, chunk_lock) in self.chunks.iter_mut() {
            chunk_lock.write().set_active(chunk_position == active_chunk_position);
        }
    }
}
