use bevy::prelude::Commands;
use common::chunks::{chunk_position::ChunkPosition, utils::SectionsData};

use super::chunks::chunks_map::ChunkMap;

#[derive(Default)]
pub struct WorldManager {
    slug: String,
    chunks_map: Box<ChunkMap>,
}

impl WorldManager {
    pub fn new(slug: String) -> Self {
        Self {
            slug,
            chunks_map: Default::default(),
        }
    }

    pub fn get_chunks_map(&self) -> &Box<ChunkMap> {
        &self.chunks_map
    }

    pub fn get_slug(&self) -> &String {
        &self.slug
    }

    pub fn load_chunk(&mut self, chunk_position: ChunkPosition, sections: SectionsData) {
        self.chunks_map.load_chunk(chunk_position, sections);
    }

    pub fn unload_chunk(&mut self, _commands: &mut Commands, _chunk_position: &ChunkPosition) {}
}
