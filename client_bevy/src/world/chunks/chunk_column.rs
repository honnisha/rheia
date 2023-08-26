use arrayvec::ArrayVec;
use bevy::prelude::{PbrBundle, Transform};
use common::{chunks::chunk_position::ChunkPosition, CHUNK_SIZE};
use common::chunks::utils::SectionsData;
use common::VERTICAL_SECTIONS;
use parking_lot::RwLock;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use super::chunk_section::ChunkSection;

pub type ColumnDataLockType = Arc<RwLock<SectionsData>>;

pub(crate) type ChunkGeneratorType = ArrayVec<PbrBundle, VERTICAL_SECTIONS>;

pub struct ChunkColumn {
    chunk_position: ChunkPosition,
    world_slug: String,
    data: ColumnDataLockType,
    sended: Arc<AtomicBool>,

    sections: ArrayVec<ChunkSection, VERTICAL_SECTIONS>,
}

impl ChunkColumn {
    pub fn new(chunk_position: ChunkPosition, world_slug: String, data: SectionsData) -> Self {
        Self {
            chunk_position,
            world_slug: world_slug,
            data: Arc::new(RwLock::new(data)),
            sended: Arc::new(AtomicBool::new(false)),
            sections: Default::default(),
        }
    }

    pub fn get_transform(&self, y: u8) -> Transform {
        Transform::from_xyz(
            self.chunk_position.x as f32 * CHUNK_SIZE as f32 - 1_f32,
            (y * CHUNK_SIZE) as f32,
            self.chunk_position.z as f32 * CHUNK_SIZE as f32 - 1_f32,
        )
    }

    pub fn get_world_slug(&self) -> &String {
        &self.world_slug
    }

    pub fn get_data(&self) -> &ColumnDataLockType {
        &self.data
    }

    pub fn is_sended(&self) -> bool {
        self.sended.load(Ordering::Relaxed)
    }

    pub fn set_sended(&self) {
        self.sended.store(true, Ordering::Relaxed);
    }

    pub(crate) fn insert_section(&mut self, section: ChunkSection) {
        self.sections.push(section);
    }
}
