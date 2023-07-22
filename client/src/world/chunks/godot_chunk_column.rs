use arrayvec::ArrayVec;
use godot::prelude::*;
use common::VERTICAL_SECTIONS;

use super::godot_chunk_section::ChunkSection;

/// Vertical section, contains vertical section
/// with VERTICAL_SECTIONS chunks sections
#[derive(GodotClass)]
#[class(base=Node)]
pub struct ChunkColumn {
    #[base]
    pub base: Base<Node>,
    sections: ArrayVec<ChunkSection, VERTICAL_SECTIONS>,
}

impl ChunkColumn {
    pub fn create(base: Base<Node>) -> Self {
        Self {
            base,
            sections: Default::default(),
        }
    }
}

#[godot_api]
impl NodeVirtual for ChunkColumn {
    /// For default godot init; only Self::create is using
    fn init(base: Base<Node>) -> Self {
        Self::create(base)
    }

    fn ready(&mut self) {
    }
}
