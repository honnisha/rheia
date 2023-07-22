use common::blocks::{block_info::BlockInfo, blocks_storage::BlockType};
use godot::{prelude::*, engine::MeshInstance3D};
use ndshape::{ConstShape3u32, ConstShape};

pub type ChunkShape = ConstShape3u32<16, 16, 16>;
pub type ChunkBordersShape = ConstShape3u32<18, 18, 18>;

pub type ChunkData = [BlockInfo; ChunkShape::SIZE as usize];
pub type ChunkDataBordered = [BlockType; ChunkBordersShape::SIZE as usize];

/// Chunk section, one of the chunk column
/// Contains mesh and data of the chunk section blocks
#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct ChunkSection {
    #[base]
    pub base: Base<Node3D>,
    mesh: Option<Gd<MeshInstance3D>>,
    loaded: bool,
}

impl ChunkSection {
    pub fn create(base: Base<Node3D>) -> Self {
        Self {
            base,
            mesh: None,
            loaded: false
        }
    }
}

#[godot_api]
impl NodeVirtual for ChunkSection {
    /// For default godot init; only Self::create is using
    fn init(base: Base<Node3D>) -> Self {
        Self::create(base)
    }

    fn ready(&mut self) {
    }
}
