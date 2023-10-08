use ahash::AHashMap;
use common::{
    blocks::block_info::BlockInfo,
    chunks::{block_position::BlockPosition, chunk_position::ChunkPosition, utils::SectionsData},
};
use godot::{engine::Material, prelude::*};
use log::error;
use parking_lot::RwLock;
use std::rc::Rc;
use std::sync::Arc;
use std::{cell::RefCell, time::Duration};

use crate::{
    utils::textures::texture_mapper::TextureMapper,
    world::{
        godot_world::{get_default_material, World},
        physics_handler::PhysicsContainer,
        world_manager::TextureMapperType,
    },
};

use super::{
    chunk::{Chunk, ColumnDataType},
    chunk_generator::{generate_chunk, spawn_chunk},
    near_chunk_data::NearChunksData,
};

const LIMIT_CHUNK_SPAWN_PER_FRAME: i32 = 1;
pub type ChunksType = AHashMap<ChunkPosition, Rc<RefCell<Chunk>>>;

/// Container of all chunk sections
#[derive(GodotClass)]
#[class(base=Node)]
pub struct ChunksContainer {
    #[base]
    pub(crate) base: Base<Node>,
    chunks: ChunksType,
    texture_mapper: TextureMapperType,
    material: Gd<Material>,
    physics_container: PhysicsContainer,
}

impl ChunksContainer {
    pub fn create(
        base: Base<Node>,
        texture_mapper: TextureMapperType,
        material: Gd<Material>,
        physics_container: PhysicsContainer,
    ) -> Self {
        Self {
            base,
            chunks: Default::default(),
            texture_mapper,
            material,
            physics_container,
        }
    }

    pub fn get_chunks_count(&self) -> usize {
        self.chunks.len()
    }

    pub fn get_chunk(&self, chunk_position: &ChunkPosition) -> Option<Rc<RefCell<Chunk>>> {
        match self.chunks.get(chunk_position) {
            Some(c) => Some(c.clone()),
            None => None,
        }
    }

    pub fn get_chunk_column_data(&self, chunk_position: &ChunkPosition) -> Option<ColumnDataType> {
        match self.chunks.get(chunk_position) {
            Some(c) => Some(c.borrow().get_chunk_data().clone()),
            None => None,
        }
    }

    pub fn modify_block(&self, _global_pos: &BlockPosition, _block_info: BlockInfo) {
        todo!();
    }

    pub fn load_chunk(&mut self, chunk_position: ChunkPosition, sections: SectionsData) {
        if self.chunks.contains_key(&chunk_position) {
            error!(
                "Network sended chunk to load, but it already exists: {}",
                chunk_position
            );
            return;
        }

        let chunk = Chunk::create(sections);
        self.chunks.insert(chunk_position.clone(), Rc::new(RefCell::new(chunk)));
    }

    pub fn unload_chunk(&mut self, chunks_positions: Vec<ChunkPosition>) {
        for chunk_position in chunks_positions {
            let mut unloaded = false;
            if let Some(chunk) = self.chunks.remove(&chunk_position) {
                if let Some(c) = chunk.borrow_mut().get_chunk_column_mut().as_mut() {
                    c.bind_mut().base.queue_free();
                }
                unloaded = true;
            }
            if !unloaded {
                error!("Unload chunk not found: {}", chunk_position);
            }
        }
    }

    /// Send new recieved chunks to load (render)
    fn send_chunks_to_load(&self) {
        let now = std::time::Instant::now();

        let mut count = 0;
        let iter = self
            .chunks
            .iter()
            .filter(|&(_chunk_position, chunk)| !chunk.borrow().is_sended());
        for (chunk_position, chunk) in iter {
            let c = chunk.borrow();

            let near_chunks_data = NearChunksData::new(&self.chunks, &chunk_position);

            // Load only if all chunks around are loaded
            if !near_chunks_data.is_full() {
                continue;
            }

            // One chunk section
            generate_chunk(
                near_chunks_data,
                c.get_chunk_data().clone(),
                c.update_tx.clone(),
                self.texture_mapper.clone(),
                self.material.instance_id(),
                chunk_position.clone(),
                self.physics_container.clone(),
            );
            c.set_sended();
            count += 1;
        }

        let elapsed = now.elapsed();
        if elapsed > Duration::from_millis(10) {
            println!(
                "ChunksContainer.SEND_chunks_to_load process: {:.2?} count:{}",
                elapsed, count
            );
        }
    }

    /// Retrieving loaded chunks to add them to the root node
    fn spawn_loaded_chunks(&mut self) {
        let now = std::time::Instant::now();
        let mut count = 0;
        for (chunk_position, chunk) in self.chunks.iter() {
            if count >= LIMIT_CHUNK_SPAWN_PER_FRAME && LIMIT_CHUNK_SPAWN_PER_FRAME != -1_i32 {
                continue;
            }

            let mut c = chunk.borrow_mut();
            if c.is_sended() && !c.is_loaded() {
                for data in c.update_rx.clone().drain() {
                    let new_chunk_col = spawn_chunk(data, chunk_position, &mut self.base);
                    c.set_chunk_column(new_chunk_col);
                    c.set_loaded();
                    count += 1;
                }
            }
        }

        let elapsed = now.elapsed();
        if elapsed > Duration::from_millis(5) {
            println!(
                "ChunksContainer.SPAWN_loaded_chunks process: {:.2?} count:{}",
                elapsed, count
            );
        }
    }
}

#[godot_api]
impl NodeVirtual for ChunksContainer {
    /// For default godot init; only World::create is using
    fn init(base: Base<Node>) -> Self {
        Self::create(
            base,
            Arc::new(RwLock::new(TextureMapper::new())),
            get_default_material(),
            PhysicsContainer::default(),
        )
    }

    fn process(&mut self, _delta: f64) {
        self.send_chunks_to_load();
        self.spawn_loaded_chunks();
    }
}
