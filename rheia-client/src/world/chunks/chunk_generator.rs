use crate::world::worlds_manager::{BlockStorageType, TextureMapperType};

use super::{
    chunk_data_formatter::format_chunk_data_with_boundaries, chunks_map::ChunkLock,
    mesh::mesh_generator::generate_chunk_geometry, near_chunk_data::NearChunksData,
};
use common::VERTICAL_SECTIONS;
use flume::Sender;
use godot::obj::InstanceId;

/// Generate chunk data in separate thread
/// and send gd instance id to the main thread to add_child it to the main tree
pub(crate) fn generate_chunk(
    chunk_column: ChunkLock,
    chunks_near: NearChunksData,
    chunks_loaded: Sender<ChunkLock>,
    material_instance_id: InstanceId,

    texture_mapper: TextureMapperType,
    block_storage: BlockStorageType,
) {
    rayon::spawn(move || {
        let data = chunk_column.read().get_chunk_lock().clone();

        chunk_column.read().spawn_sections(&material_instance_id);
        for y in 0..VERTICAL_SECTIONS {
            let (bordered_chunk_data, mesh_count) =
                format_chunk_data_with_boundaries(Some(&chunks_near), &data, &*block_storage.read(), y).unwrap();

            if mesh_count > 0 {
                let geometry =
                    generate_chunk_geometry(&texture_mapper.read(), &bordered_chunk_data, &block_storage.read());

                let mut chunk_section = chunk_column.read().get_chunk_section(&y);
                chunk_section.bind_mut().set_new_geometry(geometry);
            }
        }

        chunks_loaded
            .send(chunk_column.clone())
            .expect("chunks_loaded channel poisoned");
    });
}
