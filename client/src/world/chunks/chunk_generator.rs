use super::{
    chunk_data_formatter::format_chunk_data_with_boundaries, godot_chunk_column::ChunkColumn,
    godot_chunk_section::ChunkSection, godot_chunks_container::ColumnDataType,
    mesh::mesh_generator::generate_chunk_geometry, near_chunk_data::NearChunksData,
};
use crate::{
    entities::position::GodotPositionConverter,
    world::{physics_handler::PhysicsController, world_manager::TextureMapperType},
};
use arrayvec::ArrayVec;
use common::{chunks::chunk_position::ChunkPosition, CHUNK_SIZE, VERTICAL_SECTIONS};
use flume::Sender;
use godot::{engine::Material, prelude::*};
use log::error;
use rapier3d::prelude::ColliderBuilder;

type SectionsCollidersType = ArrayVec<Option<ColliderBuilder>, VERTICAL_SECTIONS>;
pub(crate) type ChunksGenerationType = (InstanceId, SectionsCollidersType);

/// Generate chunk in separate thread
pub(crate) fn generate_chunk(
    chunks_near: NearChunksData,
    data: ColumnDataType,
    update_tx: Sender<ChunksGenerationType>,
    texture_mapper: TextureMapperType,
    material_instance_id: InstanceId,
    chunk_position: ChunkPosition,
) {
    rayon::spawn(move || {
        let material: Gd<Material> = Gd::from_instance_id(material_instance_id);
        let mut column = Gd::<ChunkColumn>::with_base(|base| ChunkColumn::create(base, chunk_position));
        let instance_id = column.instance_id().clone();

        let mut c = column.bind_mut();
        let name = GodotString::from(format!("ChunkColumn {}", chunk_position));
        c.base.set_name(name);

        for y in 0..VERTICAL_SECTIONS {
            let mut section =
                Gd::<ChunkSection>::with_base(|base| ChunkSection::create(base, material.share(), y as u8));

            let name = GodotString::from(format!("Section {}", y));
            section.bind_mut().base.set_name(name.clone());

            c.base.add_child(section.share().upcast());
            section
                .bind_mut()
                .base
                .set_position(Vector3::new(0.0, y as f32 * CHUNK_SIZE as f32 - 1_f32, 0.0));

            c.sections.push(section);
        }

        let t = texture_mapper.read();
        let mut sections_colliders: SectionsCollidersType = Default::default();
        for y in 0..VERTICAL_SECTIONS {
            let bordered_chunk_data = format_chunk_data_with_boundaries(Some(&chunks_near), &data, y);

            // Create test sphere
            // let bordered_chunk_data = get_test_sphere();

            let geometry = generate_chunk_geometry(&t, &bordered_chunk_data);
            let mut section = c.sections[y].bind_mut();
            section.update_mesh(geometry.mesh_ist);
            sections_colliders.push(geometry.collider);
        }
        if let Err(e) = update_tx.send((instance_id, sections_colliders)) {
            error!("Send chunk {} to spawn error: {:?}", chunk_position, e);
        }
    });
}

/// Spawn chunk from main thread
pub(crate) fn spawn_chunk(
    mut data: ChunksGenerationType,
    chunk_position: &ChunkPosition,
    base: &mut Base<Node>,
    physics: &mut PhysicsController,
) -> Gd<ChunkColumn> {
    let mut column: Gd<ChunkColumn> = Gd::from_instance_id(data.0);
    let chunk_pos_vector = GodotPositionConverter::get_chunk_position_vector(&chunk_position);
    base.add_child(column.share().upcast());

    let mut y = 0;
    for collider in data.1.drain(..) {
        let section_position = Vector3::new(
            chunk_pos_vector.x,
            y as f32 * CHUNK_SIZE as f32 - 1_f32,
            chunk_pos_vector.z,
        );
        if let Some(c) = collider {
            physics.create_mesh(c, &section_position);
        }
        y += 1;
    }

    column.bind_mut().base.set_global_position(chunk_pos_vector);
    column
}
