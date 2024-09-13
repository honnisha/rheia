use arrayvec::ArrayVec;
use common::{
    blocks::block_info::BlockInfo,
    chunks::{block_position::ChunkBlockPosition, chunk_position::ChunkPosition},
    CHUNK_SIZE, VERTICAL_SECTIONS,
};
use godot::{engine::Material, prelude::*};
use network::utils::SectionsData;
use parking_lot::RwLock;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::world::{physics::PhysicsProxy, worlds_manager::TextureMapperType};

use super::{
    chunk_data_formatter::format_chunk_data_with_boundaries, chunk_section::ChunkSection,
    mesh::mesh_generator::generate_chunk_geometry, near_chunk_data::NearChunksData,
};

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
    base_id: InstanceId,

    chunk_position: ChunkPosition,
    data: ColumnDataLockType,

    // Is chunk spawned on base
    loaded: Arc<AtomicBool>,

    pub material_instance_id: InstanceId,
    pub texture_mapper: TextureMapperType,
}

impl ChunkColumn {
    pub fn create(
        chunk_position: ChunkPosition,
        data: SectionsData,
        material_instance_id: InstanceId,
        texture_mapper: TextureMapperType,
    ) -> Self {
        let chunk_base = Gd::<ChunkBase>::from_init_fn(|base| ChunkBase::create(base));

        Self {
            base_id: chunk_base.instance_id(),

            chunk_position,
            data: Arc::new(RwLock::new(data)),
            loaded: Arc::new(AtomicBool::new(false)),

            material_instance_id,
            texture_mapper,
        }
    }

    pub fn get_base(&self) -> Gd<ChunkBase> {
        let base: Gd<ChunkBase> = Gd::from_instance_id(self.base_id);
        base
    }

    pub fn spawn_sections(&self) {
        assert!(!self.is_loaded(), "Chunk cannot spawn sections twice!");

        let mut chunk_base = self.get_base();

        let material: Gd<Material> = Gd::from_instance_id(self.material_instance_id);
        chunk_base.bind_mut().spawn_sections(&self.chunk_position, material);
    }

    pub fn generate_section_geometry(&self, chunks_near: &NearChunksData, y: usize) {
        let data = self.get_chunk_data().clone();
        let bordered_chunk_data = format_chunk_data_with_boundaries(Some(&chunks_near), &data, y);

        let mut chunk_base = self.get_base();
        let mut c = chunk_base.bind_mut();

        let t = self.texture_mapper.read();
        let geometry = generate_chunk_geometry(&t, &bordered_chunk_data);
        let mut section = c.sections[y].bind_mut();

        section.set_new_geometry(geometry);
    }

    pub fn free(&mut self) {
        if self.is_loaded() {
            let mut base = self.get_base();
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

    pub fn get_chunk_position(&self) -> Vector3 {
        Vector3::new(
            self.chunk_position.x as f32 * CHUNK_SIZE as f32 - 1_f32,
            -1_f32,
            self.chunk_position.z as f32 * CHUNK_SIZE as f32 - 1_f32,
        )
    }

    pub fn spawn_loaded_chunk(&mut self) {
        let mut base = self.get_base();
        let mut c = base.bind_mut();

        // It must be updated in main thread because of
        // ERROR: Condition "!is_inside_tree()" is true. Returning: Transform3D()
        c.base_mut().set_global_position(self.get_chunk_position());
        self.set_loaded();
    }

    pub fn update_geometry(&mut self, physics: &PhysicsProxy) {
        let mut base = self.get_base();
        let mut c = base.bind_mut();

        for section in c.sections.iter_mut() {
            if section.bind().need_update_geometry {
                section.bind_mut().update_geometry(physics);
            }
        }
    }

    /// Deactivates chunks that are far away from the player
    pub fn set_active(&mut self, state: bool) {
        if self.is_loaded() {
            let mut base = self.get_base();
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
