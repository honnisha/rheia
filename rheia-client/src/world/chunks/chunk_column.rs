use arrayvec::ArrayVec;
use common::{
    blocks::block_info::BlockInfo,
    chunks::{block_position::ChunkBlockPosition, chunk_position::ChunkPosition},
    CHUNK_SIZE, VERTICAL_SECTIONS,
};
use godot::{classes::Material, prelude::*};
use network::messages::SectionsData;
use parking_lot::RwLock;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use super::chunk_section::ChunkSection;

type SectionsType = ArrayVec<Gd<ChunkSection>, VERTICAL_SECTIONS>;

pub type ColumnDataLockType = Arc<RwLock<SectionsData>>;

#[derive(GodotClass)]
#[class(no_init, tool, base=Node3D)]
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
        let name = format!("ChunkColumn {}", chunk_position);
        self.base_mut().set_name(&name);

        for y in 0..VERTICAL_SECTIONS {
            let mut section = Gd::<ChunkSection>::from_init_fn(|base| {
                ChunkSection::create(base, material.clone(), y as u8, chunk_position.clone())
            });

            let name = format!("Section {}", y);
            section.bind_mut().base_mut().set_name(&name);

            self.base_mut().add_child(&section);
            let pos = section.bind().get_section_local_position();
            section.bind_mut().base_mut().set_position(pos);

            self.sections.push(section);
        }
    }
}

/// Vertical section, contains all vertical sections
/// with VERTICAL_SECTIONS chunks sections
pub struct ChunkColumn {
    base_id: InstanceId,

    chunk_position: ChunkPosition,
    data: ColumnDataLockType,

    // Is chunk spawned on base
    loaded: Arc<AtomicBool>,
}

impl ChunkColumn {
    pub fn create(chunk_position: ChunkPosition, data: SectionsData) -> Self {
        let chunk_base = Gd::<ChunkBase>::from_init_fn(|base| ChunkBase::create(base));

        Self {
            base_id: chunk_base.instance_id(),

            chunk_position,
            data: Arc::new(RwLock::new(data)),
            loaded: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn get_base(&self) -> Gd<ChunkBase> {
        let base: Gd<ChunkBase> = Gd::from_instance_id(self.base_id);
        base
    }

    /// Spawning ChunkColumn
    /// all 0..VERTICAL_SECTIONS from bottom to top
    /// and add them as a childs to the base node of chunk column
    pub fn spawn_sections(&self, material_instance_id: &InstanceId) {
        assert!(!self.is_loaded(), "Chunk cannot spawn sections twice!");

        let mut chunk_base = self.get_base();

        let material: Gd<Material> = Gd::from_instance_id(*material_instance_id);
        chunk_base.bind_mut().spawn_sections(&self.chunk_position, material);
    }

    pub fn get_chunk_section(&self, y: &usize) -> Gd<ChunkSection> {
        assert!(
            *y < VERTICAL_SECTIONS,
            "get_chunk_section_mut y cannot be more than VERTICAL_SECTIONS"
        );
        let mut chunk_base = self.get_base();
        let c = chunk_base.bind_mut();
        c.sections[*y].clone()
    }

    pub fn free(&mut self) {
        if self.is_loaded() {
            let mut base = self.get_base();
            base.bind_mut().base_mut().queue_free();
        }
    }

    pub fn get_chunk_lock(&self) -> &ColumnDataLockType {
        &self.data
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded.load(Ordering::Relaxed)
    }

    pub fn set_loaded(&self) {
        self.loaded.store(true, Ordering::Relaxed);
    }

    pub fn get_chunk_position(&self) -> Vector3 {
        Vector3::new(
            self.chunk_position.x as f32 * CHUNK_SIZE as f32 - 1_f32,
            -1_f32,
            self.chunk_position.z as f32 * CHUNK_SIZE as f32 - 1_f32,
        )
    }

    pub fn spawn_loaded_chunk(&self) {
        let mut base = self.get_base();
        let mut c = base.bind_mut();

        // It must be updated in main thread because of
        // ERROR: Condition "!is_inside_tree()" is true. Returning: Transform3D()
        c.base_mut().set_global_position(self.get_chunk_position());
        self.set_loaded();
    }

    pub fn change_block_info(&mut self, section: u32, chunk_block: ChunkBlockPosition, new_block_info: BlockInfo) {
        if section > VERTICAL_SECTIONS as u32 {
            panic!("Tried to change block in section {section} more than max {VERTICAL_SECTIONS}");
        }

        let mut d = self.data.write();
        d[section as usize].insert(chunk_block, new_block_info);
    }
}
