use ahash::AHashMap;
use common::{
    blocks::block_info::BlockInfo,
    chunks::{block_position::BlockPosition, chunk_position::ChunkPosition, utils::SectionsData},
    VERTICAL_SECTIONS,
};
use flume::Sender;
use godot::{engine::Material, prelude::*};
use log::error;
use parking_lot::RwLock;
use spiral::ManhattanIterator;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Instant;

use crate::{
    entities::position::GodotPositionConverter,
    main_scene::CHUNKS_DISTANCE,
    utils::textures::texture_mapper::TextureMapper,
    world::world_manager::{get_default_material, TextureMapperType, WorldManager},
};

use super::{
    chunk_data_formatter::format_chunk_data_with_boundaries,
    godot_chunk_column::{ChunkColumn, ChunksGeometryType},
    mesh::mesh_generator::generate_chunk_geometry,
};

pub type ColumnDataType = Arc<RwLock<SectionsData>>;

/// Tool for storing near chunks
pub struct NearChunksData {
    pub forward: Option<ColumnDataType>,
    pub behind: Option<ColumnDataType>,
    pub left: Option<ColumnDataType>,
    pub right: Option<ColumnDataType>,
}

impl NearChunksData {
    fn new(chunks: &ChunksType, pos: &ChunkPosition) -> Self {
        Self {
            forward: NearChunksData::get_data(chunks, &ChunkPosition::new(pos.x - 1, pos.z)),
            behind: NearChunksData::get_data(chunks, &ChunkPosition::new(pos.x + 1, pos.z)),
            left: NearChunksData::get_data(chunks, &ChunkPosition::new(pos.x, pos.z - 1)),
            right: NearChunksData::get_data(chunks, &ChunkPosition::new(pos.x, pos.z + 1)),
        }
    }

    pub fn is_full(&self) -> bool {
        self.forward.is_some() && self.behind.is_some() && self.left.is_some() && self.right.is_some()
    }

    fn get_data(chunks: &ChunksType, pos: &ChunkPosition) -> Option<ColumnDataType> {
        match chunks.get(pos) {
            Some(c) => Some(c.borrow().data.clone()),
            None => None,
        }
    }
}

/// Container of godot chunk entity and blocks data
pub struct Chunk {
    chunk_column: Gd<ChunkColumn>,
    data: ColumnDataType,
}

impl Chunk {
    fn create(chunk_column: Gd<ChunkColumn>, data: ColumnDataType) -> Self {
        Self { chunk_column, data }
    }

    pub fn get_chunk_column(&self) -> &Gd<ChunkColumn> {
        &self.chunk_column
    }
}

type ChunksType = AHashMap<ChunkPosition, Rc<RefCell<Chunk>>>;

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
        let now = Instant::now();

        if self.chunks.contains_key(&chunk_position) {
            error!(
                "Network sended chunk to load, but it already exists: {}",
                chunk_position
            );
            return;
        }

        let mut column = Gd::<ChunkColumn>::with_base(|base| {
            ChunkColumn::create(base, self.material.share(), chunk_position.clone())
        });

        let name = GodotString::from(format!("ChunkColumn {}", chunk_position));
        column.bind_mut().set_name(name.clone());
        let index = column.bind().get_index().clone();

        self.base.add_child(column.upcast());
        column = self.base.get_child(index).unwrap().cast::<ChunkColumn>();

        column
            .bind_mut()
            .set_global_position(GodotPositionConverter::get_chunk_position_vector(&chunk_position));

        let chunk = Chunk::create(column, Arc::new(RwLock::new(sections)));

        self.chunks.insert(chunk_position.clone(), Rc::new(RefCell::new(chunk)));

        let elapsed = now.elapsed();
        println!("Chunk {} load: {:.2?}", chunk_position, elapsed);
    }

    pub fn unload_chunk(&mut self, chunks_positions: Vec<ChunkPosition>) {
        for chunk_position in chunks_positions {
            if let Some(chunk) = self.chunks.remove(&chunk_position) {
                chunk.borrow_mut().chunk_column.bind_mut().queue_free();
            } else {
                error!("Unload chunk not found: {}", chunk_position);
            }
        }
    }

    fn update_mesh(
        chunks_near: NearChunksData,
        data: ColumnDataType,
        update_mesh_tx: Sender<ChunksGeometryType>,
        texture_mapper: TextureMapperType,
    ) {
        rayon::spawn(move || {
            let mut geometry_array: ChunksGeometryType = Default::default();
            let t = texture_mapper.read();
            for y in 0..VERTICAL_SECTIONS {
                let bordered_chunk_data = format_chunk_data_with_boundaries(Some(&chunks_near), &data, y);

                // Create test sphere
                // let bordered_chunk_data = get_test_sphere();

                let new_geometry = generate_chunk_geometry(&t, &bordered_chunk_data);
                geometry_array.push(new_geometry);
            }
            update_mesh_tx.send(geometry_array).unwrap();
        });
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
                if chunk.borrow().chunk_column.bind().is_sended() {
                    continue;
                }

                let near_chunks_data = NearChunksData::new(&self.chunks, &chunk_position);

                // Load only if all chunks around are loaded
                if !near_chunks_data.is_full() {
                    continue;
                }

                ChunksContainer::update_mesh(
                    near_chunks_data,
                    chunk.borrow().data.clone(),
                    chunk.borrow().chunk_column.bind().update_mesh_tx.clone(),
                    self.texture_mapper.clone(),
                );
                chunk.borrow().chunk_column.bind().set_sended();
            }
        }
    }
}
