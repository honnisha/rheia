use crate::world::worlds_manager::WorldsManager;
use bevy::prelude::{Event, Events, ResMut};
use common::chunks::{chunk_position::ChunkPosition, utils::SectionsData};
use log::error;

#[derive(Event)]
pub struct ChunkLoadedEvent {
    world_slug: String,
    chunk_position: ChunkPosition,
    sections: SectionsData,
}

impl ChunkLoadedEvent {
    pub fn new(world_slug: String, chunk_position: ChunkPosition, sections: SectionsData) -> Self {
        Self {
            world_slug,
            chunk_position,
            sections,
        }
    }
}

pub fn on_chunk_loaded(
    mut chunk_loaded_event: ResMut<Events<ChunkLoadedEvent>>,
    mut worlds_manager: ResMut<WorldsManager>,
) {
    for event in chunk_loaded_event.drain() {
        match worlds_manager.get_world_mut() {
            Some(world) => {
                if event.world_slug != *world.get_slug() {
                    log::error!(
                        target: "network",
                        "Tried to load chunk {} for non existed world {}",
                        event.chunk_position, event.world_slug
                    );
                    continue;
                }
                world.load_chunk(event.chunk_position, event.sections);
            }
            None => {
                error!("unload_chunk tried to run without a world");
                continue;
            }
        }
    }
}
