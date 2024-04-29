use common::{
    chunks::chunk_position::ChunkPosition, network::messages::Vector3 as NetworkVector3, utils::fix_chunk_loc_pos,
    CHUNK_SIZE,
};
use godot::prelude::Vector3 as GDVector3;

pub struct GodotPositionConverter;

impl GodotPositionConverter {
    pub fn vector_network_from_gd(g: &GDVector3) -> NetworkVector3 {
        NetworkVector3 { x: g.x, y: g.y, z: g.z }
    }

    pub fn vector_gd_from_network(n: &NetworkVector3) -> GDVector3 {
        GDVector3 { x: n.x, y: n.y, z: n.z }
    }

    pub fn get_chunk_y_local(y: u8) -> f32 {
        y as f32 * CHUNK_SIZE as f32 - 1_f32
    }

    /// Minus one because chunk contains boundaries
    pub fn get_gd_from_chunk_position(chunk_position: &ChunkPosition) -> GDVector3 {
        GDVector3::new(
            chunk_position.x as f32 * CHUNK_SIZE as f32 - 1_f32,
            0_f32,
            chunk_position.z as f32 * CHUNK_SIZE as f32 - 1_f32,
        )
    }
}
