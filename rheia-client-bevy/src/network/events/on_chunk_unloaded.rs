use bevy::prelude::{Commands, Event, Events, ResMut};
use common::chunks::chunk_position::ChunkPosition;
use log::error;

use crate::world::worlds_manager::WorldsManager;

#[derive(Event)]
pub struct ChunkUnloadedEvent {
    world_slug: String,
    chunks_positions: Vec<ChunkPosition>,
}

impl ChunkUnloadedEvent {
    pub fn new(world_slug: String, chunks_positions: Vec<ChunkPosition>) -> Self {
        Self {
            world_slug,
            chunks_positions,
        }
    }
}

pub fn on_chunk_unloaded(
    mut commands: Commands,
    mut chunk_unloaded_event: ResMut<Events<ChunkUnloadedEvent>>,
    mut worlds_manager: ResMut<WorldsManager>,
) {
    for event in chunk_unloaded_event.drain() {
        match worlds_manager.get_world_mut() {
            Some(world) => {
                if event.world_slug != *world.get_slug() {
                    log::error!(target: "network", "Tried to unload chunks for non existed world {}", event.world_slug);
                    continue;
                }
                for chunk_position in event.chunks_positions.iter() {
                    world.unload_chunk(&mut commands, chunk_position);
                }
            }
            None => {
                error!("load_chunk tried to run without a world");
                continue;
            }
        }
    }
}
