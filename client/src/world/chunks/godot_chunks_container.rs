use ahash::AHashMap;
use common::{
    blocks::block_info::BlockInfo,
    chunks::{block_position::BlockPosition, chunk_position::ChunkPosition, utils::SectionsData},
};
use godot::{engine::Material, prelude::*};
use log::error;
use parking_lot::RwLock;
use spiral::ManhattanIterator;
use std::{cell::RefCell, time::Duration};
use std::rc::Rc;
use std::sync::Arc;

use crate::{
    entities::position::GodotPositionConverter,
    main_scene::CHUNKS_DISTANCE,
    utils::textures::texture_mapper::TextureMapper,
    world::{
        godot_world::World,
        world_manager::{get_default_material, TextureMapperType, WorldManager},
    },
};

use super::{
    chunk::{Chunk, ColumnDataType},
    chunk_generator::{generate_chunk, spawn_chunk},
    near_chunk_data::NearChunksData,
};

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
}

impl ChunksContainer {
    pub fn create(base: Base<Node>, texture_mapper: TextureMapperType, material: Gd<Material>) -> Self {
        Self {
            base,
            chunks: Default::default(),
            texture_mapper,
            material,
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
}

#[godot_api]
impl NodeVirtual for ChunksContainer {
    /// For default godot init; only World::create is using
    fn init(base: Base<Node>) -> Self {
        Self::create(
            base,
            Arc::new(RwLock::new(TextureMapper::new())),
            get_default_material(),
        )
    }

    fn process(&mut self, _delta: f64) {
        let now = std::time::Instant::now();

        for (chunk_position, chunk) in self.chunks.iter() {
            let c = chunk.borrow();
            if c.is_sended() {
                continue;
            }

            let near_chunks_data = NearChunksData::new(&self.chunks, &chunk_position);

            // Load only if all chunks around are loaded
            if !near_chunks_data.is_full() {
                continue;
            }

            generate_chunk(
                near_chunks_data,
                c.get_chunk_data().clone(),
                c.update_tx.clone(),
                self.texture_mapper.clone(),
                self.material.instance_id(),
                chunk_position.clone(),
            );
            c.set_sended();
        }

        let elapsed = now.elapsed();
        if elapsed > Duration::from_millis(3) {
            println!("ChunksContainer SEND process: {:.2?}", elapsed);
        }

        let now1 = std::time::Instant::now();

        let mut world = self.base.get_parent().unwrap().cast::<World>();
        for (chunk_position, chunk) in self.chunks.iter() {
            let mut c = chunk.borrow_mut();
            if c.is_sended() && !c.is_loaded() {
                for data in c.update_rx.clone().drain() {
                    let mut w = world.bind_mut();
                    let physics = w.get_physics_mut();
                    c.set_chunk_column(spawn_chunk(data, chunk_position, &mut self.base, physics));
                    c.set_loaded()
                }
            }
        }

        let elapsed = now1.elapsed();
        if elapsed > Duration::from_millis(3) {
            println!("ChunksContainer spawn process: {:.2?}", elapsed);
        }
    }
}
