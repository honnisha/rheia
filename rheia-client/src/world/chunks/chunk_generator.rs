use super::{
    chunk::ColumnDataType, chunk_data_formatter::format_chunk_data_with_boundaries, godot_chunk_column::ChunkColumn,
    godot_chunk_section::ChunkSection, mesh::mesh_generator::generate_chunk_geometry, near_chunk_data::NearChunksData,
};
use crate::{
    main_scene::PhysicsContainerType, utils::position::GodotPositionConverter, world::world_manager::TextureMapperType,
};
use common::{chunks::chunk_position::ChunkPosition, physics::physics::PhysicsContainer, VERTICAL_SECTIONS};
use flume::Sender;
use godot::{engine::Material, prelude::*};
use log::error;

pub(crate) type ChunksGenerationType = InstanceId;

/// Generate chunk data in separate thread
/// and send gd instance id to the main thread to add_child it to the main tree
pub(crate) fn generate_chunk(
    chunks_near: NearChunksData,
    data: ColumnDataType,
    update_tx: Sender<ChunksGenerationType>,
    texture_mapper: TextureMapperType,
    material_instance_id: InstanceId,
    chunk_position: ChunkPosition,
    physics_container: PhysicsContainerType,
) {
    //rayon::spawn(move || {
        let material: Gd<Material> = Gd::from_instance_id(material_instance_id);
        let mut column = Gd::<ChunkColumn>::from_init_fn(|base| ChunkColumn::create(base, chunk_position));
        let instance_id = column.instance_id().clone();

        {
            let mut c = column.bind_mut();

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
        }
        if let Err(e) = update_tx.send(instance_id) {
            error!("Send chunk {} to spawn error: {:?}", chunk_position, e);
        }
    //});
}

/// Recieved gd instance id from channel and
/// spawn chunk from main thread
pub(crate) fn spawn_chunk(
    id: ChunksGenerationType,
    chunk_position: &ChunkPosition,
    base: &mut Gd<Node>,
) -> Gd<ChunkColumn> {
    let now = std::time::Instant::now();

    let mut column: Gd<ChunkColumn> = Gd::from_instance_id(id);
    base.add_child(column.clone().upcast());

    {
        let mut c = column.bind_mut();

        // It must be updated in main thread because of
        // ERROR: Condition "!is_inside_tree()" is true. Returning: Transform3D()
        let chunk_pos_vector = GodotPositionConverter::get_gd_from_chunk_position(&chunk_position);
        c.base_mut().set_global_position(chunk_pos_vector);

        for section in c.sections.iter_mut() {
            section.bind_mut().sync();
        }
    }

    let elapsed = now.elapsed();
    if elapsed > std::time::Duration::from_millis(5) {
        log::debug!(target: "chunks", "spawn_chunk process: {:.2?}", elapsed);
    }
    column
}
