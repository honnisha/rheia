use super::{chunks_map::ChunkLock, near_chunk_data::NearChunksData};
use common::VERTICAL_SECTIONS;

/// Generate chunk data in separate thread
/// and send gd instance id to the main thread to add_child it to the main tree
pub(crate) fn generate_chunk(
    chunk_column: ChunkLock,
    chunks_near: NearChunksData,
    chunks_loaded: flume::Sender<ChunkLock>,
) {
    rayon::spawn(move || {
        let c = chunk_column.read();

        c.spawn_sections();
        for y in 0..VERTICAL_SECTIONS {
            c.generate_section_geometry(&chunks_near, y);
        }

        chunks_loaded.send(chunk_column.clone()).unwrap();
    });
}
