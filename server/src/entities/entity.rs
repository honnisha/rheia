use bevy_ecs::prelude::Component;
use common::{
    chunks::{block_position::BlockPositionTrait, chunk_position::ChunkPosition},
    utils::fix_chunk_loc_pos,
};
use uuid::Uuid;

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
}

impl BlockPositionTrait for Position {
    fn get_chunk_position(&self) -> ChunkPosition {
        ChunkPosition::new(fix_chunk_loc_pos(self.x as i32), fix_chunk_loc_pos(self.z as i32))
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
