use super::{
    chunk_column::{ChunkColumn, ColumnDataLockType},
    chunk_data_formatter::format_chunk_data_with_boundaries,
    chunk_section::ChunkSection,
    mesh::mesh_generator::generate_chunk_geometry,
    near_chunk_data::NearChunksData,
};
use crate::{main_scene::PhysicsContainerType, world::worlds_manager::TextureMapperType};
use common::{chunks::chunk_position::ChunkPosition, physics::physics::PhysicsContainer, VERTICAL_SECTIONS};
use godot::{engine::Material, prelude::*};

use flume::Sender;

/// Generate chunk data in separate thread
/// and send gd instance id to the main thread to add_child it to the main tree
pub(crate) fn generate_chunk(
    chunk_column_id: InstanceId,
    chunks_near: NearChunksData,
    data: ColumnDataLockType,
    texture_mapper: TextureMapperType,
    material_instance_id: InstanceId,
    chunk_position: ChunkPosition,
    physics_container: PhysicsContainerType,
    chunks_loaded: Sender<ChunkPosition>,
) {
    rayon::spawn(move || {
        let material: Gd<Material> = Gd::from_instance_id(material_instance_id);
        let mut chunk_column: Gd<ChunkColumn> = Gd::from_instance_id(chunk_column_id);

        let mut c = chunk_column.bind_mut();

        let name = GString::from(format!("ChunkColumn {}", chunk_position));
        c.base_mut().set_name(name);

        for y in 0..VERTICAL_SECTIONS {
            let physics_entity = physics_container.create_static();

            let mut section = Gd::<ChunkSection>::from_init_fn(|base| {
                ChunkSection::create(base, material.clone(), y as u8, physics_entity, chunk_position.clone())
            });

            let name = GString::from(format!("Section {}", y));
            section.bind_mut().base_mut().set_name(name.clone());

            c.base_mut().add_child(section.clone().upcast());
            let pos = section.bind().get_section_local_position();
            section.bind_mut().base_mut().set_position(pos);

            c.sections.push(section);
        }

        let t = texture_mapper.read();
        for y in 0..VERTICAL_SECTIONS {
            let bordered_chunk_data = format_chunk_data_with_boundaries(Some(&chunks_near), &data, y);

            // Create test sphere
            // let bordered_chunk_data = get_test_sphere();

            let geometry = generate_chunk_geometry(&t, &bordered_chunk_data);
            let mut section = c.sections[y].bind_mut();

            section.send_to_update_mesh(geometry);
        }
        chunks_loaded.send(chunk_position).unwrap();
    });
}
