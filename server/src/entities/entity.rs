use bevy_ecs::prelude::Component;
use common::CHUNK_SIZE;
use uuid::Uuid;

use crate::worlds::chunks::chunk_position::ChunkPosition;

#[derive(Component)]
pub struct Indentifier(Uuid);

impl Default for Indentifier {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Component)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32,
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn get_chunk_position(&self) -> ChunkPosition {
        ChunkPosition::new(
            fix_chunk_loc_pos(self.x as i32),
            fix_chunk_loc_pos(self.z as i32)
        )
    }
}

#[derive(Component)]
pub struct NetworkComponent {
    pub client_id: u64,
}

impl NetworkComponent {
    pub fn new(client_id: u64) -> Self {
        Self { client_id }
    }
}


fn fix_chunk_loc_pos(p: i32) -> i32 {
    if p < 0 {
        return (p + 1_i32) / CHUNK_SIZE + -1_i32;
    }
    return p / CHUNK_SIZE;
}
