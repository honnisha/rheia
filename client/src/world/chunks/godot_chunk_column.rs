use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use arrayvec::ArrayVec;
use common::{chunks::chunk_position::ChunkPosition, VERTICAL_SECTIONS};
use flume::{Receiver, Sender};
use godot::{engine::Material, prelude::*};

use crate::{
    entities::position::GodotPositionConverter, utils::mesh::mesh_generator::Geometry,
    world::world_manager::get_default_material,
};

use super::godot_chunk_section::ChunkSection;

type SectionsType = ArrayVec<Gd<ChunkSection>, VERTICAL_SECTIONS>;

pub(crate) type ChunksGeometryType = ArrayVec<Geometry, VERTICAL_SECTIONS>;

/// Vertical section, contains vertical section
/// with VERTICAL_SECTIONS chunks sections
#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct ChunkColumn {
    #[base]
    pub base: Base<Node3D>,
    sections: SectionsType,
    sended: Arc<AtomicBool>,
    loaded: bool,
    chunk_position: ChunkPosition,

    pub update_mesh_tx: Sender<ChunksGeometryType>,
    update_mesh_rx: Receiver<ChunksGeometryType>,

    material: Gd<Material>,
}

impl ChunkColumn {
    pub fn create(base: Base<Node3D>, material: Gd<Material>, chunk_position: ChunkPosition) -> Self {
        let (update_mesh_tx, update_mesh_rx) = flume::bounded(1);
        Self {
            base,
            sections: Default::default(),
            loaded: false,
            sended: Arc::new(AtomicBool::new(false)),
            chunk_position,

            update_mesh_tx: update_mesh_tx,
            update_mesh_rx: update_mesh_rx,

            material,
        }
    }

    pub fn is_sended(&self) -> bool {
        self.sended.load(Ordering::Relaxed)
    }

    pub fn set_sended(&self) {
        self.sended.store(true, Ordering::Relaxed);
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    fn spawn_chunk_section(
        base: &mut Base<Node3D>,
        y: usize,
        material: &Gd<Material>,
        chunk_position: &ChunkPosition,
    ) -> Gd<ChunkSection> {
        let mut section = Gd::<ChunkSection>::with_base(|base| ChunkSection::create(base, material.share()));

        let name = GodotString::from(format!("Section {}", y));
        section.bind_mut().set_name(name.clone());
        let index = section.bind().get_index().clone();

        base.add_child(section.upcast());
        let mut section = base.get_child(index).unwrap().cast::<ChunkSection>();
        section
            .bind_mut()
            .set_global_position(GodotPositionConverter::get_chunk_section_position_vector(
                chunk_position,
                y as u8,
            ));
        section
    }
}

#[godot_api]
impl NodeVirtual for ChunkColumn {
    /// For default godot init; only Self::create is using
    fn init(base: Base<Node3D>) -> Self {
        Self::create(base, get_default_material(), ChunkPosition::default())
    }

    fn ready(&mut self) {
        for y in 0..VERTICAL_SECTIONS {
            self.sections.push(ChunkColumn::spawn_chunk_section(
                &mut self.base,
                y,
                &self.material,
                &self.chunk_position,
            ));
        }
    }

    fn process(&mut self, _delta: f64) {
        for mut section_geometry in self.update_mesh_rx.drain() {
            let y = 0;
            for geometry in section_geometry.drain(..) {
                self.sections[y].bind_mut().update_mesh(geometry.mesh_ist);
            }
            self.loaded = true;
        }
    }
}
