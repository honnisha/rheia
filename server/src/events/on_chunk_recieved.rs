use bevy::prelude::Event;
use bevy_ecs::prelude::Events;
use bevy_ecs::system::{Res, ResMut};
use common::chunks::chunk_position::ChunkPosition;

use crate::network::clients_container::ClientsContainer;

#[derive(Event)]
pub struct ChunkRecievedEvent {
    client_id: u64,
    chunk_positions: Vec<ChunkPosition>,
}

impl ChunkRecievedEvent {
    pub fn new(client_id: u64, chunk_positions: Vec<ChunkPosition>) -> Self {
        Self {
            client_id,
            chunk_positions,
        }
    }
}

pub fn on_chunk_recieved(
    mut chunk_recieved_events: ResMut<Events<ChunkRecievedEvent>>,
    clients: Res<ClientsContainer>,
) {
    for event in chunk_recieved_events.drain() {
        let client = clients.get(&event.client_id);
        client.mark_chunks_as_recieved(event.chunk_positions);
    }
}
