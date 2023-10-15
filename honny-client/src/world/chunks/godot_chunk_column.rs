use arrayvec::ArrayVec;
use common::{chunks::chunk_position::ChunkPosition, VERTICAL_SECTIONS};
use godot::prelude::*;

use super::godot_chunk_section::ChunkSection;

type SectionsType = ArrayVec<Gd<ChunkSection>, VERTICAL_SECTIONS>;

/// Vertical section, contains all vertical sections
/// with VERTICAL_SECTIONS chunks sections
#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct ChunkColumn {
    #[base]
    pub base: Base<Node3D>,
    pub sections: SectionsType,
    active: bool,
}

impl ChunkColumn {
    pub fn create(base: Base<Node3D>, _chunk_position: ChunkPosition) -> Self {
        Self {
            base,
            sections: Default::default(),
            active: false,
        }
    }

    pub fn change_activity(&mut self, active: bool) {
        if self.active != active {
            self.active = active;
            for section in self.sections.iter_mut() {
                section.bind_mut().change_activity(active);
            }
        }
    }
}

#[godot_api]
impl NodeVirtual for ChunkColumn {
    /// For default godot init; only Self::create is using
    fn init(base: Base<Node3D>) -> Self {
        Self::create(base, ChunkPosition::default())
    }
}
