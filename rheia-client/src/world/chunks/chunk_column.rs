use arrayvec::ArrayVec;
use common::{
    chunks::{chunk_position::ChunkPosition, utils::SectionsData},
    VERTICAL_SECTIONS,
};
use godot::prelude::*;
use parking_lot::RwLock;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::utils::bridge::IntoGodotVector;

use super::chunk_section::ChunkSection;

type SectionsType = ArrayVec<Gd<ChunkSection>, VERTICAL_SECTIONS>;

pub type ColumnDataLockType = Arc<RwLock<SectionsData>>;

#[derive(GodotClass)]
#[class(no_init, base=Node3D)]
pub struct ChunkBase {
    pub base: Base<Node3D>,

    pub sections: SectionsType,
}

impl ChunkBase {
    pub fn create(base: Base<Node3D>) -> Self {
        Self {
            base,
            sections: Default::default()
        }
    }
}

/// Vertical section, contains all vertical sections
/// with VERTICAL_SECTIONS chunks sections
pub struct ChunkColumn {
    base: Option<Gd<ChunkBase>>,

    chunk_position: ChunkPosition,
    data: ColumnDataLockType,
    sended: Arc<AtomicBool>,
    loaded: Arc<AtomicBool>,
}

impl ChunkColumn {
    pub fn create(chunk_position: ChunkPosition, data: SectionsData) -> Self {
        Self {
            base: Default::default(),
            chunk_position,
            data: Arc::new(RwLock::new(data)),
            sended: Arc::new(AtomicBool::new(false)),
            loaded: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn free(&mut self) {
        if let Some(base) = self.base.as_mut() {
            base.bind_mut().base_mut().queue_free();
        }
    }

    pub fn get_chunk_data(&self) -> &ColumnDataLockType {
        &self.data
    }

    pub fn is_sended(&self) -> bool {
        self.sended.load(Ordering::Relaxed)
    }

    pub fn set_sended(&self) {
        self.sended.store(true, Ordering::Relaxed);
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded.load(Ordering::Relaxed)
    }

    pub fn set_loaded(&self) {
        self.loaded.store(true, Ordering::Relaxed);
    }

    pub fn spawn_loaded_chunk(&mut self, mut chunk_base: Gd<ChunkBase>) {
        self.base = Some(chunk_base.clone());
        let mut c = chunk_base.bind_mut();

        // It must be updated in main thread because of
        // ERROR: Condition "!is_inside_tree()" is true. Returning: Transform3D()
        c.base_mut().set_global_position(self.chunk_position.to_godot());

        for section in c.sections.iter_mut() {
            if section.bind().need_sync {
                section.bind_mut().chunk_section_sync();
            }
        }
        self.set_loaded();
    }
}
