use common::{chunks::chunk_position::ChunkPosition, utils::fix_chunk_loc_pos, CHUNK_SIZE};
use godot::prelude::Vector3;

pub struct GodotPositionConverter;

impl GodotPositionConverter {
    pub fn vec3_to_array(pos: &Vector3) -> [f32; 3] {
        [pos.x, pos.y, pos.z]
    }

    pub fn vec3_from_array(pos: &[f32; 3]) -> Vector3 {
        Vector3::new(pos[0], pos[1], pos[2])
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
    pub fn _get_chunk_section_position_vector(chunk_position: &ChunkPosition, y: u8) -> Vector3 {
        Vector3::new(
            chunk_position.x as f32 * CHUNK_SIZE as f32 - 1_f32,
            y as f32 * CHUNK_SIZE as f32 - 1_f32,
            chunk_position.z as f32 * CHUNK_SIZE as f32 - 1_f32,
        )
    }
}
