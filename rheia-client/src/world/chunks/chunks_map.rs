use ahash::AHashMap;
use common::chunks::{chunk_position::ChunkPosition, utils::SectionsData};
use godot::{engine::Material, prelude::*};
use log::error;
use std::cell::RefCell;
use std::rc::Rc;

use crate::{
    main_scene::PhysicsContainerType, utils::bridge::IntoGodotVector, world::worlds_manager::TextureMapperType,
};

use super::{
    chunk_column::{ChunkColumn, ColumnDataLockType},
    chunk_generator::generate_chunk,
    near_chunk_data::NearChunksData,
};
use flume::{unbounded, Receiver, Sender};

pub type ChunksType = AHashMap<ChunkPosition, Gd<ChunkColumn>>;

/// Container of all chunk sections
#[derive(GodotClass)]
#[class(no_init, base=Node)]
pub struct ChunkMap {
    pub(crate) base: Base<Node>,

    // Hash map with chunk columns
    chunks: ChunksType,

    texture_mapper: TextureMapperType,
    material: Gd<Material>,
    physics_container: PhysicsContainerType,

    sended_chunks: Rc<RefCell<Vec<ChunkPosition>>>,
    loaded_chunks: (Sender<ChunkPosition>, Receiver<ChunkPosition>),
}

impl ChunkMap {
    pub fn create(
        base: Base<Node>,
        texture_mapper: TextureMapperType,
        material: Gd<Material>,
        physics_container: PhysicsContainerType,
    ) -> Self {
        Self {
            base,
            chunks: Default::default(),
            texture_mapper,
            material,
            physics_container,
            sended_chunks: Default::default(),
            loaded_chunks: unbounded(),
        }
    }

    pub fn get_chunks_count(&self) -> usize {
        self.chunks.len()
    }

    pub fn get_chunk(&self, chunk_position: &ChunkPosition) -> Option<Gd<ChunkColumn>> {
        match self.chunks.get(chunk_position) {
            Some(c) => Some(c.clone()),
            None => None,
        }
    }

    pub fn get_chunk_column_data(&self, chunk_position: &ChunkPosition) -> Option<ColumnDataLockType> {
        match self.chunks.get(chunk_position) {
            Some(c) => Some(c.bind().get_chunk_data().clone()),
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

        let chunk_column = Gd::<ChunkColumn>::from_init_fn(|base| ChunkColumn::create(base, chunk_position, sections));
        self.chunks.insert(chunk_position.clone(), chunk_column);
        self.sended_chunks.borrow_mut().push(chunk_position);
    }

    pub fn unload_chunk(&mut self, chunk_position: ChunkPosition) {
        let mut unloaded = false;
        if let Some(mut chunk_column) = self.chunks.remove(&chunk_position) {
            chunk_column.bind_mut().base_mut().queue_free();
            unloaded = true;
        }
        if !unloaded {
            error!("Unload chunk not found: {}", chunk_position);
        }
    }

    /// Send new recieved chunks to load (render)
    fn send_chunks_to_load(&mut self) {
        self.sended_chunks.borrow_mut().retain(|chunk_position| {
            let near_chunks_data = NearChunksData::new(&self.chunks, &chunk_position);

            // Load only if all chunks around are loaded
            if !near_chunks_data.is_full() {
                return true;
            }

            let chunk_column = self.get_chunk(&chunk_position).unwrap();
            generate_chunk(
                chunk_column.instance_id(),
                near_chunks_data,
                self.get_chunk_column_data(chunk_position).unwrap(),
                self.texture_mapper.clone(),
                self.material.instance_id(),
                chunk_position.clone(),
                self.physics_container.clone(),
                self.loaded_chunks.0.clone(),
            );
            chunk_column.bind().set_sended();
            return false;
        });
    }

    /// Retrieving loaded chunks to add them to the root node
    fn spawn_loaded_chunks(&mut self) {
        let mut base = self.base_mut().clone();
        for chunk_position in self.loaded_chunks.1.drain() {
            let mut chunk_column = self.get_chunk(&chunk_position).unwrap();

            base.add_child(chunk_column.clone().upcast());

            let mut c = chunk_column.bind_mut();

            // It must be updated in main thread because of
            // ERROR: Condition "!is_inside_tree()" is true. Returning: Transform3D()
            c.base_mut().set_global_position(chunk_position.to_godot());

            for section in c.sections.iter_mut() {
                if section.bind().need_sync {
                    section.bind_mut().chunk_section_sync();
                }
            }
            c.set_loaded();
        }
    }
}

#[godot_api]
impl INode for ChunkMap {
    fn process(&mut self, _delta: f64) {
        self.send_chunks_to_load();
        self.spawn_loaded_chunks();
    }
}
