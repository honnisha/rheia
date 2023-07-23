use common::{chunks::chunk_position::ChunkPosition, utils::fix_chunk_loc_pos, CHUNK_SIZE};
use godot::prelude::Vector3;

pub struct GodotPositionConverter;

impl GodotPositionConverter {
    fn get_chunk_position(pos: Vector3) -> ChunkPosition {
        ChunkPosition::new(fix_chunk_loc_pos(pos.x as i64), fix_chunk_loc_pos(pos.z as i64))
    }

    /// Minus one because chunk contains boundaries
    pub fn get_chunk_position_vector(chunk_position: &ChunkPosition) -> Vector3 {
        Vector3::new(
            chunk_position.x as f32 * CHUNK_SIZE as f32 - 1_f32,
            0_f32,
            chunk_position.z as f32 * CHUNK_SIZE as f32 - 1_f32,
        )
    }

    /// Minus one because chunk contains boundaries
    pub fn get_chunk_section_position_vector(chunk_position: &ChunkPosition, y: u8) -> Vector3 {
        Vector3::new(
            chunk_position.x as f32 * CHUNK_SIZE as f32 - 1_f32,
            y as f32 * CHUNK_SIZE as f32 - 1_f32,
            chunk_position.z as f32 * CHUNK_SIZE as f32 - 1_f32,
        )
    }
}
