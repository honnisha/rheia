use ahash::AHashMap;
use common::{
    blocks::block_info::BlockInfo,
    chunks::{block_position::BlockPosition, chunk_position::ChunkPosition},
    network::NetworkSectionType,
    VERTICAL_SECTIONS,
};
use flume::Sender;
use godot::{engine::Material, prelude::*};
use log::trace;
use parking_lot::RwLock;
use std::sync::Arc;

use crate::{
    utils::{mesh::mesh_generator::generate_chunk_geometry, textures::texture_mapper::TextureMapper},
    world::world_manager::{get_default_material, TextureMapperType},
};

use super::{
    chunk_data_formatter::format_chunk_data_with_boundaries,
    godot_chunk_column::{ChunkColumn, ChunksGeometryType},
};

pub type ColumnDataType = Arc<RwLock<NetworkSectionType>>;

/// Tool for storing near chunks
pub struct NearChunksData {
    pub forward: Option<ColumnDataType>,
    pub behind: Option<ColumnDataType>,
    pub left: Option<ColumnDataType>,
    pub right: Option<ColumnDataType>,
}

impl NearChunksData {
    fn new(chunks: &AHashMap<ChunkPosition, Chunk>, pos: &ChunkPosition) -> Self {
        Self {
            forward: NearChunksData::get_data(chunks, &ChunkPosition::new(pos.x + 1, pos.z)),
            behind: NearChunksData::get_data(chunks, &ChunkPosition::new(pos.x - 1, pos.z)),
            right: NearChunksData::get_data(chunks, &ChunkPosition::new(pos.x, pos.z + 1)),
            left: NearChunksData::get_data(chunks, &ChunkPosition::new(pos.x, pos.z - 1)),
        }
    }

    fn get_data(chunks: &AHashMap<ChunkPosition, Chunk>, pos: &ChunkPosition) -> Option<ColumnDataType> {
        match chunks.get(pos) {
            Some(c) => Some(c.data.clone()),
            None => None,
        }
    }
}

/// Container of godot chunk entity and blocks data
struct Chunk {
    chunk: Gd<ChunkColumn>,
    data: ColumnDataType,
}

impl Chunk {
    fn create(chunk: Gd<ChunkColumn>, data: ColumnDataType) -> Self {
        Self { chunk, data }
    }
}

/// Container of all chunk sections
#[derive(GodotClass)]
#[class(base=Node)]
pub struct ChunksContainer {
    #[base]
    base: Base<Node>,
    chunks: AHashMap<ChunkPosition, Chunk>,
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

    pub fn get_chunk_column_data(&self, chunk_position: &ChunkPosition) -> Option<ColumnDataType> {
        match self.chunks.get(chunk_position) {
            Some(c) => Some(c.data.clone()),
            None => None,
        }
    }

    pub fn modify_block(&self, global_pos: &BlockPosition, block_info: BlockInfo) {
        todo!();
    }

    pub fn load_chunk(&mut self, chunk_position: ChunkPosition, sections: NetworkSectionType) {
        let mut column = Gd::<ChunkColumn>::with_base(|base| ChunkColumn::create(base, self.material.share()));

        let name = GodotString::from(format!("ChunkColumn {}", chunk_position));
        column.bind_mut().set_name(name.clone());
        let index = column.bind().get_index().clone();

        self.base.add_child(column.upcast());
        column = self.base.get_child(index).unwrap().cast::<ChunkColumn>();

        self.chunks.insert(
            chunk_position.clone(),
            Chunk::create(column, Arc::new(RwLock::new(sections))),
        );
        trace!("Chunk created at position {}", chunk_position);
    }

    fn update_mesh(
        chunks_near: NearChunksData,
        data: ColumnDataType,
        update_mesh_tx: Sender<ChunksGeometryType>,
        texture_mapper: TextureMapperType,
    ) {
        let mut geometry_array: ChunksGeometryType = Default::default();
        let t = texture_mapper.read();
        for y in 0..VERTICAL_SECTIONS {
            let bordered_chunk_data = format_chunk_data_with_boundaries(&chunks_near, &data, y);
            let new_geometry = generate_chunk_geometry(&t, &bordered_chunk_data);
            geometry_array.push(new_geometry);
        }
        update_mesh_tx.send(geometry_array).unwrap();
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
        for (chunk_position, chunk) in self.chunks.iter() {
            if !chunk.chunk.bind().is_sended() {
                let near_chunks_data = NearChunksData::new(&self.chunks, chunk_position);
                ChunksContainer::update_mesh(
                    near_chunks_data,
                    chunk.data.clone(),
                    chunk.chunk.bind().update_mesh_tx.clone(),
                    self.texture_mapper.clone(),
                );
                trace!("Mesh {} generated", chunk_position);
                chunk.chunk.bind().set_sended();
            }
        }
    }
}
