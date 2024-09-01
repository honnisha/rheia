use super::{
    chunk_data_formatter::format_chunk_data_with_boundaries, chunks_map::ChunkLock,
    mesh::mesh_generator::generate_chunk_geometry, near_chunk_data::NearChunksData,
};
use common::VERTICAL_SECTIONS;
use godot::{engine::Material, prelude::*};

/// Generate chunk data in separate thread
/// and send gd instance id to the main thread to add_child it to the main tree
pub(crate) fn generate_chunk(
    chunk_column: ChunkLock,
    chunks_near: NearChunksData,
    chunks_loaded: flume::Sender<ChunkLock>,
) {
    rayon::spawn(move || {
        let chunk_column_lock = chunk_column.read();
        let data = chunk_column_lock.get_chunk_data().clone();

        let mut chunk_base = chunk_column_lock.get_base();
        let mut c = chunk_base.bind_mut();

        let material: Gd<Material> = Gd::from_instance_id(chunk_column_lock.material_instance_id);
        c.spawn_sections(chunk_column_lock.get_chunk_position(), material);

        let t = chunk_column_lock.texture_mapper.read();
        for y in 0..VERTICAL_SECTIONS {
            let bordered_chunk_data = format_chunk_data_with_boundaries(Some(&chunks_near), &data, y);

            let geometry = generate_chunk_geometry(&t, &bordered_chunk_data);
            let mut section = c.sections[y].bind_mut();

            section.send_to_update_mesh(geometry);
        }

        chunks_loaded.send(chunk_column.clone()).unwrap();
    });
}
