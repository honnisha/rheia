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

use super::chunk_section::ChunkSection;

type SectionsType = ArrayVec<Gd<ChunkSection>, VERTICAL_SECTIONS>;

pub type ColumnDataLockType = Arc<RwLock<SectionsData>>;

/// Vertical section, contains all vertical sections
/// with VERTICAL_SECTIONS chunks sections
#[derive(GodotClass)]
#[class(no_init, base=Node3D)]
pub struct ChunkColumn {
    pub base: Base<Node3D>,

    data: ColumnDataLockType,
    sended: Arc<AtomicBool>,
    loaded: Arc<AtomicBool>,

    pub sections: SectionsType,
}

impl ChunkColumn {
    pub fn create(base: Base<Node3D>, _chunk_position: ChunkPosition, data: SectionsData) -> Self {
        Self {
            base,
            data: Arc::new(RwLock::new(data)),
            sended: Arc::new(AtomicBool::new(false)),
            loaded: Arc::new(AtomicBool::new(false)),
            sections: Default::default(),
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
}
