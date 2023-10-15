use arrayvec::ArrayVec;
use common::{chunks::chunk_position::ChunkPosition, VERTICAL_SECTIONS};
use godot::prelude::*;

use super::godot_chunk_section::ChunkSection;

type SectionsType = ArrayVec<Gd<ChunkSection>, VERTICAL_SECTIONS>;

pub const DEFAULT_CHUNK_ACTIVITY: bool = false;

/// Vertical section, contains all vertical sections
/// with VERTICAL_SECTIONS chunks sections
#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct ChunkColumn {
    #[base]
    pub base: Base<Node3D>,
    pub sections: SectionsType,
    active: bool,
    chunk_position: ChunkPosition,
}

impl ChunkColumn {
    pub fn create(base: Base<Node3D>, chunk_position: ChunkPosition) -> Self {
        Self {
            base,
            sections: Default::default(),
            active: DEFAULT_CHUNK_ACTIVITY,
            chunk_position,
        }
    }

    /// Updates the activity of the chunk. Sets it to pause if it is not active.
    pub fn change_activity(&mut self, active: bool) {
        if self.active != active {
            self.active = active;
            // println!("Set activity chunk {}: {}", self.chunk_position, active);
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
