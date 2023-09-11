use bevy::prelude::Event;
use bevy_ecs::prelude::Events;
use bevy_ecs::system::{Res, ResMut};
use common::chunks::chunk_position::ChunkPosition;

use crate::network::clients_container::ClientsContainer;
use crate::network::server::NetworkContainer;

#[derive(Event)]
pub struct ChunkRecievedEvent {
    client_id: u64,
    chunk_position: ChunkPosition,
}

impl ChunkRecievedEvent {
    pub fn new(client_id: u64, chunk_position: ChunkPosition) -> Self {
        Self {
            client_id,
            chunk_position,
        }
    }
}

pub fn on_chunk_recieved(
    mut chunk_recieved_events: ResMut<Events<ChunkRecievedEvent>>,
    clients: Res<ClientsContainer>,
) {
    for event in chunk_recieved_events.drain() {
        let client = clients.get(&event.client_id);
        client.mark_chunk_as_recieved(event.chunk_position);
    }
}
