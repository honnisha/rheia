use ahash::AHashMap;
use common::{
    blocks::block_info::BlockInfo,
    chunks::{block_position::BlockPosition, chunk_position::ChunkPosition, utils::SectionsData},
};
use flume::{Receiver, Sender};
use godot::{engine::Material, prelude::*};
use log::error;
use parking_lot::RwLock;
use spiral::ManhattanIterator;
use std::rc::Rc;
use std::sync::Arc;
use std::{
    cell::RefCell,
    sync::atomic::{AtomicBool, Ordering},
};

use crate::{
    entities::position::GodotPositionConverter,
    main_scene::CHUNKS_DISTANCE,
    utils::textures::texture_mapper::TextureMapper,
    world::world_manager::{get_default_material, TextureMapperType, WorldManager},
};

use super::{
    chunk_generator::{generate_chunk, spawn_chunk, ChunksGenerationType},
    godot_chunk_column::ChunkColumn,
    near_chunk_data::NearChunksData,
};

pub type ColumnDataType = Arc<RwLock<SectionsData>>;

/// Container of godot chunk entity and blocks data
pub struct Chunk {
    // Godot entity
    chunk_column: Option<Gd<ChunkColumn>>,

    // Chunk data
    data: ColumnDataType,

    update_tx: Sender<ChunksGenerationType>,
    update_rx: Receiver<ChunksGenerationType>,

    sended: Arc<AtomicBool>,
    loaded: Arc<AtomicBool>,
}

impl Chunk {
    fn create(data: ColumnDataType) -> Self {
        let (update_tx, update_rx) = flume::bounded(1);
        Self {
            chunk_column: None,
            data,
            sended: Arc::new(AtomicBool::new(false)),
            loaded: Arc::new(AtomicBool::new(false)),
            update_tx: update_tx,
            update_rx: update_rx,
        }
    }

    pub fn get_chunk_column(&self) -> Option<&Gd<ChunkColumn>> {
        match self.chunk_column.as_ref() {
            Some(c) => Some(c),
            None => None,
        }
    }

    pub fn get_chunk_data(&self) -> &ColumnDataType {
        &self.data
    }

    pub fn is_sended(&self) -> bool {
        self.sended.load(Ordering::Relaxed)
    }

    pub fn set_sended(&self) {
        self.sended.store(true, Ordering::Relaxed);
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded.load(Ordering::Relaxed)
    }

    pub fn set_loaded(&self) {
        self.loaded.store(true, Ordering::Relaxed);
    }
}

pub type ChunksType = AHashMap<ChunkPosition, Rc<RefCell<Chunk>>>;

/// Container of all chunk sections
#[derive(GodotClass)]
#[class(base=Node)]
pub struct ChunksContainer {
    #[base]
    base: Base<Node>,
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
            Some(c) => Some(c.borrow().data.clone()),
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

        let chunk = Chunk::create(Arc::new(RwLock::new(sections)));

        self.chunks.insert(chunk_position.clone(), Rc::new(RefCell::new(chunk)));
    }

    pub fn unload_chunk(&mut self, chunks_positions: Vec<ChunkPosition>) {
        for chunk_position in chunks_positions {
            let mut unloaded = false;
            if let Some(chunk) = self.chunks.remove(&chunk_position) {
                if let Some(c) = chunk.borrow_mut().chunk_column.as_mut() {
                    c.bind_mut().queue_free();
                    unloaded = true;
                }
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
        let world_manager = self.get_parent().unwrap().get_parent().unwrap().cast::<WorldManager>();
        let controller_positon = world_manager.bind().get_player_controller().bind().get_position();
        let current_chunk = GodotPositionConverter::get_chunk_position(&controller_positon);

        let iter = ManhattanIterator::new(current_chunk.x as i32, current_chunk.z as i32, CHUNKS_DISTANCE);
        for (x, z) in iter {
            let chunk_position = ChunkPosition::new(x as i64, z as i64);
            if let Some(chunk) = self.get_chunk(&chunk_position) {
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
                    c.data.clone(),
                    c.update_tx.clone(),
                    self.texture_mapper.clone(),
                    self.material.instance_id(),
                    chunk_position.clone()
                );
                c.set_sended();
            }
        }

        for (chunk_position, chunk) in self.chunks.iter() {
            let mut c = chunk.borrow_mut();
            if c.is_sended() && !c.is_loaded() {
                for data in c.update_rx.clone().drain() {
                    c.chunk_column = Some(spawn_chunk(data, &mut self.base));
                    c.set_loaded()
                }
            }
        }
    }
}
