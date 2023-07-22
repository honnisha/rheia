use common::{chunks::{block_position::BlockPositionTrait, chunk_position::ChunkPosition}, utils::fix_chunk_loc_pos};
use godot::prelude::Vector3;

pub struct GodotPositionConverter;

impl GodotPositionConverter {
    fn get_chunk_position(pos: Vector3) -> ChunkPosition {
        ChunkPosition::new(fix_chunk_loc_pos(pos.x as i64), fix_chunk_loc_pos(pos.z as i64))
    }
}
