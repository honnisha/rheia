use common::{chunks::chunk_position::ChunkPosition, network::messages::Vector3 as NetworkVector3, CHUNK_SIZE};
use godot::prelude::Vector3 as GDVector3;

pub trait IntoGodotVector {
    fn to_godot(&self) -> GDVector3;
}

impl IntoGodotVector for NetworkVector3 {
    fn to_godot(&self) -> GDVector3 {
        GDVector3::new(self.x, self.y, self.z)
    }
}

impl IntoGodotVector for ChunkPosition {
    fn to_godot(&self) -> GDVector3 {
        // Minus one because chunk contains boundaries
        GDVector3::new(
            self.x as f32 * CHUNK_SIZE as f32 - 1_f32,
            0_f32,
            self.z as f32 * CHUNK_SIZE as f32 - 1_f32,
        )
    }
}

pub trait IntoNetworkVector {
    fn to_network(&self) -> NetworkVector3;
}

impl IntoNetworkVector for GDVector3 {
    fn to_network(&self) -> NetworkVector3 {
        NetworkVector3::new(self.x, self.y, self.z)
    }
}

pub struct GodotPositionConverter;

impl GodotPositionConverter {
    pub fn get_chunk_y_local(y: u8) -> f32 {
        y as f32 * CHUNK_SIZE as f32 - 1_f32
    }
}
