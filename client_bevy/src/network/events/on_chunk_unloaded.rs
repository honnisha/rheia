use bevy::prelude::{Event, Events, ResMut, Commands};
use common::chunks::chunk_position::ChunkPosition;
use log::error;

use crate::world::worlds_manager::WorldsManager;

#[derive(Event)]
pub struct ChunkUnloadedEvent {
    chunks_positions: Vec<ChunkPosition>,
}

impl ChunkUnloadedEvent {
    pub fn new(chunks_positions: Vec<ChunkPosition>) -> Self {
        Self { chunks_positions }
    }
}

pub fn on_chunk_unloaded(
    mut commands: Commands,
    mut chunk_unloaded_event: ResMut<Events<ChunkUnloadedEvent>>,
    mut worlds_manager: ResMut<WorldsManager>,
) {
    for event in chunk_unloaded_event.drain() {
        match worlds_manager.get_world_mut() {
            Some(w) => {
                for chunk_position in event.chunks_positions.iter() {
                    w.unload_chunk(&mut commands, chunk_position);
                }
            }
            None => {
                error!("load_chunk tried to run without a world");
                continue;
            }
        }
    }
}
