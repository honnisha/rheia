use arrayvec::ArrayVec;
use common::{chunks::chunk_position::ChunkPosition, VERTICAL_SECTIONS};
use godot::{engine::Material, prelude::*};

use crate::world::world_manager::get_default_material;

use super::godot_chunk_section::ChunkSection;

type SectionsType = ArrayVec<Gd<ChunkSection>, VERTICAL_SECTIONS>;

/// Vertical section, contains vertical section
/// with VERTICAL_SECTIONS chunks sections
#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct ChunkColumn {
    #[base]
    pub base: Base<Node3D>,
    pub sections: SectionsType,
    chunk_position: ChunkPosition,

    material: Gd<Material>,
}

impl ChunkColumn {
    pub fn create(base: Base<Node3D>, material: Gd<Material>, chunk_position: ChunkPosition) -> Self {
        Self {
            base,
            sections: Default::default(),
            chunk_position,
            material,
        }
    }
}

#[godot_api]
impl NodeVirtual for ChunkColumn {
    /// For default godot init; only Self::create is using
    fn init(base: Base<Node3D>) -> Self {
        Self::create(base, get_default_material(), ChunkPosition::default())
    }
}
