use common::{
    CHUNK_SIZE, VERTICAL_SECTIONS,
    chunks::{
        block_position::{BlockPosition, ChunkBlockPosition},
        chunk_data::{BlockDataInfo, ChunkData},
        chunk_position::ChunkPosition,
    },
};
use godot::{classes::Material, prelude::*};
use parking_lot::RwLock;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use super::chunk_section::ChunkSection;

type SectionsGdType = Vec<Gd<ChunkSection>>;

pub type ColumnDataLockType = Arc<RwLock<ChunkData>>;

#[derive(GodotClass)]
#[class(no_init, tool, base=Node3D)]
pub struct ChunkBase {
    pub base: Base<Node3D>,

    pub sections: SectionsGdType,
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
    pub fn create(chunk_position: ChunkPosition, data: ChunkData) -> Self {
        let chunk_base = Gd::<ChunkBase>::from_init_fn(|base| ChunkBase::create(base));

        let chunk_column = Self {
            base_id: chunk_base.instance_id(),

            chunk_position,
            data: Arc::new(RwLock::new(data)),
            loaded: Arc::new(AtomicBool::new(false)),
        };

        chunk_column.get_base().set_position(chunk_column.get_position());

        chunk_column
    }

    pub(crate) fn get_base(&self) -> Gd<ChunkBase> {
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
        let chunk_base = self.get_base();
        let c = chunk_base.bind();
        c.sections[*y].clone()
    }

    pub fn free(&mut self) {
        let mut chunk_base = self.get_base();
        let mut c = chunk_base.bind_mut();
        for section in c.sections.iter_mut() {
            section.bind_mut().destory();
        }

        if self.is_loaded() {
            c.base_mut().queue_free();
        }
    }

    pub fn get_data_lock(&self) -> &ColumnDataLockType {
        &self.data
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded.load(Ordering::Relaxed)
    }

    pub fn set_loaded(&self) {
        self.loaded.store(true, Ordering::Relaxed);
    }

    pub fn get_chunk_position(&self) -> &ChunkPosition {
        &self.chunk_position
    }

    pub fn get_position(&self) -> Vector3 {
        Vector3::new(
            self.chunk_position.x as f32 * CHUNK_SIZE as f32,
            0.0,
            self.chunk_position.z as f32 * CHUNK_SIZE as f32,
        )
    }

    pub fn change_block_info(
        &mut self,
        section: u32,
        chunk_block: &ChunkBlockPosition,
        new_block_info: Option<BlockDataInfo>,
    ) {
        let mut d = self.data.write();
        d.change_block(section, &chunk_block, new_block_info);
    }

    pub fn get_block_info(&self, block_position: &BlockPosition) -> Option<BlockDataInfo> {
        let d = self.data.read();
        d.get_block_info(block_position)
    }
}
