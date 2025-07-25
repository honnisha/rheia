use crate::{
    client_scripts::resource_manager::{ResourceManager, ResourceStorage},
    utils::textures::texture_mapper::TextureMapper,
    world::{
        block_storage::BlockStorage,
        physics::PhysicsProxy,
        worlds_manager::{BlockStorageType, TextureMapperType},
    },
};
use ahash::{AHashMap, HashSet};
use common::{
    blocks::block_type::BlockContent,
    chunks::{
        block_position::{BlockPosition, BlockPositionTrait},
        chunk_data::{BlockDataInfo, ChunkData},
        chunk_position::ChunkPosition,
    },
    CHUNK_SIZE, VERTICAL_SECTIONS,
};
use godot::prelude::*;
use parking_lot::RwLock;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use super::{
    chunk_column::{ChunkColumn, ColumnDataLockType},
    chunk_data_formatter::format_chunk_data_with_boundaries,
    chunk_generator::generate_chunk,
    mesh::mesh_generator::generate_chunk_geometry,
    near_chunk_data::NearChunksData,
};
use flume::{unbounded, Receiver, Sender};

const MAX_CHUNKS_SPAWN_PER_FRAME: i32 = 6;
const LIMIT_CHUNK_LOADING_AT_A_TIME: u32 = 16;

pub type ChunkLock = Arc<RwLock<ChunkColumn>>;
pub type ChunksType = AHashMap<ChunkPosition, ChunkLock>;

/// Container of all chunk sections
#[derive(GodotClass)]
#[class(no_init, tool, base=Node)]
pub struct ChunkMap {
    pub(crate) base: Base<Node>,

    // Hash map with chunk columns
    chunks: ChunksType,

    sended_chunks: Rc<RefCell<Vec<ChunkPosition>>>,
    chunks_to_spawn: (Sender<ChunkLock>, Receiver<ChunkLock>),

    chunks_to_update: Rc<RefCell<HashSet<(ChunkPosition, usize)>>>,

    loading_chunks: RefCell<u32>,
}

#[godot_api]
impl ChunkMap {
    #[signal]
    fn chunk_recieved();
}

impl ChunkMap {
    pub fn create(base: Base<Node>) -> Self {
        Self {
            base,
            chunks: Default::default(),
            sended_chunks: Default::default(),
            chunks_to_spawn: unbounded(),
            chunks_to_update: Default::default(),
            loading_chunks: Default::default(),
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
            Some(c) => Some(c.read().get_data_lock().clone()),
            None => None,
        }
    }

    /// Create chunk column and send it to render queue
    pub fn create_chunk_column(&mut self, center: ChunkPosition, chunk_position: ChunkPosition, sections: ChunkData) {
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

        if self.sended_chunks.borrow().contains(&chunk_position) {
            panic!("sended_chunks already have chunk");
        }
        self.sended_chunks.borrow_mut().push(chunk_position);

        self.sended_chunks.borrow_mut().sort_by(|a, b| {
            a.get_distance(&center)
                .partial_cmp(&b.get_distance(&center))
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    /// Send new recieved chunks to load (render)
    ///
    /// Can be sended only if all bordered chunks are loaded
    pub fn send_chunks_to_load(
        &mut self,
        material_instance_id: InstanceId,
        texture_mapper: TextureMapperType,
        block_storage: BlockStorageType,
        physics: &PhysicsProxy,
        resource_manager: &ResourceManager,
    ) {
        if *self.loading_chunks.borrow() > LIMIT_CHUNK_LOADING_AT_A_TIME {
            return;
        }

        self.sended_chunks.borrow_mut().retain(|chunk_position| {
            if *self.loading_chunks.borrow() > LIMIT_CHUNK_LOADING_AT_A_TIME {
                return true;
            }

            let near_chunks_data = NearChunksData::new(&self.chunks, &chunk_position);

            // Load only if all chunks around are loaded
            if !near_chunks_data.is_full() {
                return true;
            }

            let Some(chunk_column) = self.get_chunk(&chunk_position) else {
                // Remove if chunk is not existing for some reason
                return false;
            };

            generate_chunk(
                chunk_column.clone(),
                near_chunks_data,
                self.chunks_to_spawn.0.clone(),
                material_instance_id,
                texture_mapper.clone(),
                block_storage.clone(),
                physics.clone(),
                resource_manager,
            );
            *self.loading_chunks.borrow_mut() += 1;
            return false;
        });
    }

    /// Retrieving loaded chunks to add them to the root node
    pub fn spawn_loaded_chunks(&mut self, physics: &PhysicsProxy) {
        let mut base = self.base_mut().clone();

        // for l in self.chunks_to_spawn.1.drain() {

        let mut i = 0;
        loop {
            if i > MAX_CHUNKS_SPAWN_PER_FRAME {
                break;
            }
            // Take only one chunk
            if let Ok(l) = self.chunks_to_spawn.1.try_recv() {
                let chunk_column = l.read();

                let mut chunk_base = chunk_column.get_base();
                base.add_child(&chunk_base);
                chunk_column.set_loaded();

                let mut c = chunk_base.bind_mut();

                for section in c.sections.iter_mut() {
                    if section.bind().is_geometry_update_needed() {
                        section.bind_mut().update_geometry(physics);
                    }
                }
                *self.loading_chunks.borrow_mut() -= 1;

                i += 1;
            } else {
                break;
            }
        }
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

    /// Changes block info and place updated chunk into the queue for an update
    pub fn edit_block(
        &self,
        position: BlockPosition,
        block_storage: &BlockStorage,
        new_block_info: Option<BlockDataInfo>,
        physics: &PhysicsProxy,
        resource_storage: &ResourceStorage,
    ) -> Result<(), String> {
        let Some(chunk_column) = self.chunks.get(&position.get_chunk_position()) else {
            panic!("edit_block chunk not found");
        };

        let (section, block_position) = position.get_block_position();
        if section > VERTICAL_SECTIONS as u32 {
            panic!("section y cannot be more than VERTICAL_SECTIONS");
        }

        if let Some(old_block_info) = chunk_column.read().get_block_info(&position) {
            let old_block_type = block_storage.get(&old_block_info.get_id()).unwrap();

            match old_block_type.get_block_content() {
                BlockContent::Texture { .. } => {
                    self.send_to_update_chunk_mesh(&position);
                }
                BlockContent::ModelCube { .. } => {
                    let chunk_column = self
                        .get_chunk(&position.get_chunk_position())
                        .expect("chunk from chunks_to_update is not found");

                    let c = chunk_column.read();
                    let mut chunk_section = c.get_chunk_section(&(section as usize));

                    let mut cs = chunk_section.bind_mut();
                    let objects_container = cs.get_objects_container_mut();
                    objects_container.bind_mut().remove(&block_position);
                }
            }
        }

        chunk_column
            .write()
            .change_block_info(section, &block_position, new_block_info.clone());

        if let Some(new_block_info) = new_block_info {
            let Some(new_block_type) = block_storage.get(&new_block_info.get_id()) else {
                return Err(format!("edit block id #{} not found", new_block_info.get_id()));
            };

            match new_block_type.get_block_content() {
                BlockContent::Texture { .. } => {
                    self.send_to_update_chunk_mesh(&position);
                }
                BlockContent::ModelCube {
                    model,
                    icon_size: _,
                    collider_type,
                } => {
                    let chunk_column = self
                        .get_chunk(&position.get_chunk_position())
                        .expect("chunk from chunks_to_update is not found");

                    let c = chunk_column.read();
                    let mut chunk_section = c.get_chunk_section(&(section as usize));

                    let mut cs = chunk_section.bind_mut();
                    let objects_container = cs.get_objects_container_mut();
                    objects_container
                        .bind_mut()
                        .create_block_model(
                            &position,
                            model,
                            collider_type,
                            Some(physics),
                            resource_storage,
                            new_block_info.get_face(),
                        )
                        .unwrap();
                }
            }
        }

        Ok(())
    }

    /// Sent to the update queue
    fn send_to_update_chunk_mesh(&self, position: &BlockPosition) {
        let (section, block_position) = position.get_block_position();

        self.chunks_to_update
            .borrow_mut()
            .insert((position.get_chunk_position(), section as usize));

        if block_position.y == 0 && section > 0 {
            self.chunks_to_update
                .borrow_mut()
                .insert((position.get_chunk_position(), section as usize - 1));
        }

        if block_position.y == CHUNK_SIZE - 1 && section < VERTICAL_SECTIONS as u32 - 1 {
            self.chunks_to_update
                .borrow_mut()
                .insert((position.get_chunk_position(), section as usize + 1));
        }

        let x = position.get_chunk_position() + ChunkPosition::new(-1, 0);
        if block_position.x == 0 && self.get_chunk(&x).is_some() {
            self.chunks_to_update.borrow_mut().insert((x, section as usize));
        }

        let x = position.get_chunk_position() + ChunkPosition::new(1, 0);
        if block_position.x == CHUNK_SIZE - 1 && self.get_chunk(&x).is_some() {
            self.chunks_to_update.borrow_mut().insert((x, section as usize));
        }

        let z = position.get_chunk_position() + ChunkPosition::new(0, -1);
        if block_position.z == 0 && self.get_chunk(&z).is_some() {
            self.chunks_to_update.borrow_mut().insert((z, section as usize));
        }

        let z = position.get_chunk_position() + ChunkPosition::new(0, 1);
        if block_position.z == CHUNK_SIZE - 1 && self.get_chunk(&z).is_some() {
            self.chunks_to_update.borrow_mut().insert((z, section as usize));
        }
    }

    /// Every frame job to update edited chunks
    pub fn update_chunks_geometry(
        &self,
        physics: &PhysicsProxy,
        block_storage: &BlockStorage,
        texture_mapper: &TextureMapper,
    ) {
        self.chunks_to_update.borrow_mut().retain(|(chunk_position, y)| {
            let chunks_near = NearChunksData::new(&self.chunks, &chunk_position);

            // Load only if all chunks around are loaded
            if !chunks_near.is_full() {
                return true;
            }

            let Some(chunk_column) = self.get_chunk(&chunk_position) else {
                // Remove if chunk is not existing for some reason
                return false;
            };

            let c = chunk_column.read();

            let data = c.get_data_lock().clone();

            let (bordered_chunk_data, _mesh_count) =
                format_chunk_data_with_boundaries(Some(&chunks_near), &data, &block_storage, y.clone()).unwrap();

            let geometry = generate_chunk_geometry(&texture_mapper, &bordered_chunk_data, &block_storage);

            let mut chunk_section = c.get_chunk_section(y);
            chunk_section.bind_mut().set_new_geometry(geometry);
            chunk_section.bind_mut().update_geometry(&physics);

            return false;
        });
    }
}
