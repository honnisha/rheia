use arrayvec::ArrayVec;
use common::{chunks::chunk_position::ChunkPosition, VERTICAL_SECTIONS};
use godot::prelude::*;

use super::chunk_section::ChunkSection;

type SectionsType = ArrayVec<Gd<ChunkSection>, VERTICAL_SECTIONS>;

/// Vertical section, contains all vertical sections
/// with VERTICAL_SECTIONS chunks sections
#[derive(GodotClass)]
#[class(no_init, base=Node3D)]
pub struct ChunkColumn {
    pub base: Base<Node3D>,
    pub sections: SectionsType,
}

impl ChunkColumn {
    pub fn create(base: Base<Node3D>, _chunk_position: ChunkPosition) -> Self {
        Self {
            base,
            sections: Default::default(),
        }
    }
}
