use std::sync::{atomic::{AtomicBool, Ordering}, Arc};

use arrayvec::ArrayVec;
use common::{VERTICAL_SECTIONS};
use flume::{Receiver, Sender};
use godot::prelude::*;

use crate::utils::mesh::mesh_generator::Geometry;

use super::godot_chunk_section::ChunkSection;

type SectionsType = ArrayVec<Gd<ChunkSection>, VERTICAL_SECTIONS>;

pub(crate) type ChunksGeometryType = ArrayVec<Geometry, VERTICAL_SECTIONS>;

/// Vertical section, contains vertical section
/// with VERTICAL_SECTIONS chunks sections
#[derive(GodotClass)]
#[class(base=Node)]
pub struct ChunkColumn {
    #[base]
    pub base: Base<Node>,
    sections: SectionsType,
    sended: Arc<AtomicBool>,
    loaded: bool,

    pub update_mesh_tx: Sender<ChunksGeometryType>,
    update_mesh_rx: Receiver<ChunksGeometryType>,
}

impl ChunkColumn {
    pub fn create(base: Base<Node>,) -> Self {
        let (update_mesh_tx, update_mesh_rx) = flume::bounded(1);
        Self {
            base,
            sections: Default::default(),
            loaded: false,
            sended: Arc::new(AtomicBool::new(false)),

            update_mesh_tx: update_mesh_tx,
            update_mesh_rx: update_mesh_rx,
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

    fn spawn_chunk_section(base: &mut Base<Node>, y: usize) -> Gd<ChunkSection> {
        let mut section = Gd::<ChunkSection>::with_base(|base| ChunkSection::create(base));

        let name = GodotString::from(format!("Section {}", y));
        section.bind_mut().set_name(name.clone());
        let index = section.bind().get_index().clone();

        base.add_child(section.upcast());
        base.get_child(index).unwrap().cast::<ChunkSection>()
    }
}

#[godot_api]
impl NodeVirtual for ChunkColumn {
    /// For default godot init; only Self::create is using
    fn init(base: Base<Node>) -> Self {
        Self::create(base)
    }

    fn ready(&mut self) {
        for y in 0..VERTICAL_SECTIONS {
            self.sections.push(ChunkColumn::spawn_chunk_section(&mut self.base, y));
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
