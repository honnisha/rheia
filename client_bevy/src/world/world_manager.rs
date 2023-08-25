use bevy::prelude::{Commands, EventWriter, PbrBundle, ResMut};
use common::chunks::{chunk_position::ChunkPosition, utils::SectionsData};

use std::borrow::Borrow;

use super::chunks::chunk_generator::GenerateChunkEvent;
use super::{
    chunks::{chunk_column::ChunkColumn, chunks_map::ChunkMap, near_chunk_data::NearChunksData},
    worlds_manager::WorldsManager,
};

pub struct WorldManagerPlugin;

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
        let chunk_column = ChunkColumn::new(chunk_position.clone(), self.get_slug().clone(), sections);
        self.chunks_map.insert(chunk_position, chunk_column);
    }

    pub fn unload_chunk(&mut self, commands: &mut Commands, chunk_position: &ChunkPosition) {}
}

pub fn chunks_loader_system(
    mut worlds_manager: ResMut<WorldsManager>,
    mut chunk_generate_event: EventWriter<GenerateChunkEvent>,
) {
    if let Some(world) = worlds_manager.get_world_mut() {
        let chunks_map: &ChunkMap = world.chunks_map.borrow();
        for (chunk_position, chunk_lock) in chunks_map.iter() {
            let c = chunk_lock.read();
            if c.is_sended() {
                continue;
            }

            let near_chunks_data = NearChunksData::new(&world.chunks_map.borrow(), &chunk_position);

            // Load only if all chunks around are loaded
            if !near_chunks_data.is_full() {
                continue;
            }

            c.set_sended();
            let e = GenerateChunkEvent::new(c.get_world_slug().clone(), chunk_position.clone(), near_chunks_data);
            chunk_generate_event.send(e);
        }
    }
}
