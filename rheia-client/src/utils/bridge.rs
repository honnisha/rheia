use common::{
    chunks::{block_position::ChunkBlockPosition, chunk_position::ChunkPosition, position::Vector3 as NetworkVector3}, utils::fix_chunk_loc_pos,
    CHUNK_SIZE,
};
use godot::prelude::Vector3 as GDVector3;

pub trait IntoGodotVector {
    fn to_godot(&self) -> GDVector3;
}

pub trait IntoNetworkVector {
    fn to_network(&self) -> NetworkVector3;
}

pub trait IntoChunkPositionVector {
    fn to_chunk_position(&self) -> ChunkPosition;
}

impl IntoGodotVector for NetworkVector3 {
    fn to_godot(&self) -> GDVector3 {
        GDVector3::new(self.x, self.y, self.z)
    }
}

impl IntoGodotVector for ChunkBlockPosition {
    fn to_godot(&self) -> GDVector3 {
        GDVector3::new(self.x as f32, self.y as f32, self.z as f32)
    }
}

impl IntoNetworkVector for GDVector3 {
    fn to_network(&self) -> NetworkVector3 {
        NetworkVector3::new(self.x, self.y, self.z)
    }
}

impl IntoChunkPositionVector for GDVector3 {
    fn to_chunk_position(&self) -> ChunkPosition {
        ChunkPosition::new(fix_chunk_loc_pos(self.x as i64), fix_chunk_loc_pos(self.z as i64))
    }
}

pub struct GodotPositionConverter;

impl GodotPositionConverter {
    pub fn get_chunk_y_local(y: u8) -> f32 {
        y as f32 * CHUNK_SIZE as f32
    }
}
