use arrayvec::ArrayVec;
use common::{chunks::chunk_position::ChunkPosition, VERTICAL_SECTIONS, CHUNK_SIZE};
use flume::Sender;
use godot::{engine::Material, prelude::*};

use crate::{entities::position::GodotPositionConverter, world::world_manager::TextureMapperType};

use super::{
    chunk_data_formatter::format_chunk_data_with_boundaries,
    godot_chunk_column::ChunkColumn,
    godot_chunks_container::ColumnDataType,
    mesh::mesh_generator::{generate_chunk_geometry, Geometry},
    near_chunk_data::NearChunksData, godot_chunk_section::ChunkSection,
};

pub(crate) type ChunksGenerationType = ArrayVec<Geometry, VERTICAL_SECTIONS>;

/// Send chunk to generation
pub(crate) fn generate_chunk(
    chunks_near: NearChunksData,
    data: ColumnDataType,
    update_mesh_tx: Sender<ChunksGenerationType>,
    texture_mapper: TextureMapperType,
) {
    rayon::spawn(move || {
        let mut geometry_array: ChunksGenerationType = Default::default();
        let t = texture_mapper.read();
        for y in 0..VERTICAL_SECTIONS {
            let bordered_chunk_data = format_chunk_data_with_boundaries(Some(&chunks_near), &data, y);

            // Create test sphere
            // let bordered_chunk_data = get_test_sphere();

            let new_geometry = generate_chunk_geometry(&t, &bordered_chunk_data);
            geometry_array.push(new_geometry);
        }
        update_mesh_tx.send(geometry_array).unwrap();
    });
}

pub(crate) fn spawn_chunk(
    data: &mut ChunksGenerationType,
    base: &mut Base<Node>,
    material: Gd<Material>,
    chunk_position: ChunkPosition,
) {
    let mut column = Gd::<ChunkColumn>::with_base(|base| ChunkColumn::create(base, material.share(), chunk_position));

    let index = {
        let mut c = column.bind_mut();
        for y in 0..VERTICAL_SECTIONS {
            let mut section = Gd::<ChunkSection>::with_base(|base| ChunkSection::create(base, material.share(), y as u8));

            let name = GodotString::from(format!("Section {}", y));
            section.bind_mut().set_name(name.clone());
            let index = section.bind().get_index().clone();

            c.base.add_child(section.upcast());
            let mut section = c.base.get_child(index).unwrap().cast::<ChunkSection>();
            section.bind_mut().create_mesh();
            section
                .bind_mut()
                .set_global_position(Vector3::new(0.0, y as f32 * CHUNK_SIZE as f32 - 1_f32, 0.0));

            c.sections.push(section);
        }

        let mut y = 0;
        for geometry in data.drain(..) {
            c.sections[y].bind_mut().update_mesh(geometry.mesh_ist);
            y += 1;
        }

        let name = GodotString::from(format!("ChunkColumn {}", chunk_position));
        c.set_name(name.clone());
        c.get_index().clone()
    };

    base.add_child(column.upcast());
    column = base.get_child(index).unwrap().cast::<ChunkColumn>();

    column
        .bind_mut()
        .set_global_position(GodotPositionConverter::get_chunk_position_vector(&chunk_position));
}
