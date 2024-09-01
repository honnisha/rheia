use arrayvec::ArrayVec;
use common::{
    blocks::block_info::BlockInfo,
    chunks::{block_position::ChunkBlockPosition, chunk_position::ChunkPosition, utils::SectionsData},
    VERTICAL_SECTIONS,
};
use godot::{engine::Material, prelude::*};
use parking_lot::RwLock;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::{utils::bridge::IntoGodotVector, world::physics::PhysicsProxy};

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
            sections: Default::default(),
        }
    }

    pub fn spawn_sections(&mut self, chunk_position: &ChunkPosition, material: Gd<Material>) {
        let name = GString::from(format!("ChunkColumn {}", chunk_position));
        self.base_mut().set_name(name);

        for y in 0..VERTICAL_SECTIONS {
            let mut section = Gd::<ChunkSection>::from_init_fn(|base| {
                ChunkSection::create(base, material.clone(), y as u8, chunk_position.clone())
            });

            let name = GString::from(format!("Section {}", y));
            section.bind_mut().base_mut().set_name(name.clone());

            self.base_mut().add_child(section.clone().upcast());
            let pos = section.bind().get_section_local_position();
            section.bind_mut().base_mut().set_position(pos);

            self.sections.push(section);
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

    pub fn is_loaded(&self) -> bool {
        self.loaded.load(Ordering::Relaxed)
    }

    pub fn set_loaded(&self) {
        self.loaded.store(true, Ordering::Relaxed);
    }

    pub fn spawn_loaded_chunk(&mut self, mut chunk_base: Gd<ChunkBase>, physics: &PhysicsProxy) {
        self.base = Some(chunk_base.clone());
        let mut c = chunk_base.bind_mut();

        // It must be updated in main thread because of
        // ERROR: Condition "!is_inside_tree()" is true. Returning: Transform3D()
        c.base_mut().set_global_position(self.chunk_position.to_godot());

        for section in c.sections.iter_mut() {
            if section.bind().need_sync {
                section.bind_mut().chunk_section_sync(physics);
            }
        }
        self.set_loaded();
    }

    /// Deactivates chunks that are far away from the player
    pub fn set_active(&mut self, state: bool) {
        if let Some(base) = self.base.as_mut() {
            for section in base.bind_mut().sections.as_mut() {
                section.bind_mut().set_active(state);
            }
        }
    }

    pub fn change_block_info(&mut self, section: u32, chunk_block: ChunkBlockPosition, new_block_info: BlockInfo) {
        if section > VERTICAL_SECTIONS as u32 {
            panic!("Tried to change block in section {section} more than max {VERTICAL_SECTIONS}");
        }

        let mut d = self.data.write();
        d[section as usize].insert(chunk_block, new_block_info);
    }
}
