use std::collections::HashMap;

use common::{chunks::chunk_position::ChunkPosition, network::messages::ChunkDataType, VERTICAL_SECTIONS};

use crate::worlds::{chunks::chunk_column::ChunkColumn, world_generator::sphere::SphereWorldGenerator};

#[test]
fn test_chunk_sphere_size() {
    let chunk_position = ChunkPosition::new(0, 0);
    let mut chunk_column = ChunkColumn::new(chunk_position, "test_world".to_string());

    let generator = SphereWorldGenerator::new(0);
    for y in 0..VERTICAL_SECTIONS {
        let mut chunk_section: ChunkDataType = HashMap::new();
        generator.generate_chunk_data(&mut chunk_section, &chunk_position, y);
        chunk_column.sections.push(Box::new(chunk_section));
    }

    let data = chunk_column.build_network_format(true);

    let encoded = bincode::serialize(&data).unwrap();
    let size = encoded.len();
    let target = 386310;
    assert!(size <= target, "Size of chunk message {} more than {} bytes", size, target);
}
