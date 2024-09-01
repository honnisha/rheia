use super::{
    chunk_column::{ChunkBase, ColumnDataLockType},
    chunk_data_formatter::format_chunk_data_with_boundaries,
    mesh::mesh_generator::generate_chunk_geometry,
    near_chunk_data::NearChunksData,
};
use crate::world::{
    worlds_manager::TextureMapperType,
};
use common::{chunks::chunk_position::ChunkPosition, VERTICAL_SECTIONS};
use godot::{engine::Material, prelude::*};

use flume::Sender;

/// Generate chunk data in separate thread
/// and send gd instance id to the main thread to add_child it to the main tree
pub(crate) fn generate_chunk(
    data: ColumnDataLockType,
    chunks_near: NearChunksData,
    texture_mapper: TextureMapperType,
    material_instance_id: InstanceId,
    chunk_position: ChunkPosition,
    chunks_loaded: Sender<(ChunkPosition, InstanceId)>,
) {
    rayon::spawn(move || {
        let material: Gd<Material> = Gd::from_instance_id(material_instance_id);

        let mut chunk_base = Gd::<ChunkBase>::from_init_fn(|base| ChunkBase::create(base));

        {
            let mut c = chunk_base.bind_mut();
            c.spawn_sections(&chunk_position, material);

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

        chunks_loaded.send((chunk_position, chunk_base.instance_id())).unwrap();
    });
}
