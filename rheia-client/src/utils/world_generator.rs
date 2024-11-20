use arrayvec::ArrayVec;
use common::{chunks::chunk_position::ChunkPosition, world_generator::default::WorldGenerator, VERTICAL_SECTIONS};
use godot::obj::Gd;
use network::messages::{ChunkDataType, SectionsData};
use spiral::ManhattanIterator;

use crate::world::world_manager::WorldManager;

pub fn generate_chunks(
    world: &mut Gd<WorldManager>,
    x: i32,
    z: i32,
    chunks_distance: u16,
    settings: serde_json::Value,
) {
    let now = std::time::Instant::now();

    let mut durations: Vec<std::time::Duration> = Default::default();

    let world_generator = WorldGenerator::create(settings).unwrap();

    let iter = ManhattanIterator::new(x, z, chunks_distance);
    for (x, z) in iter {
        let now = std::time::Instant::now();

        let chunk_pos = ChunkPosition::new(x as i64, z as i64);

        let data = generate_chunk(&world_generator, &chunk_pos);
        world.bind_mut().recieve_chunk(chunk_pos, data);
        durations.push(now.elapsed());
    }

    let mut avg = std::time::Duration::from_secs_f32(0.0);
    for i in durations.iter() {
        avg += *i;
    }
    avg = std::time::Duration::from_millis((avg.as_millis() as f32 / durations.len() as f32) as u64);

    log::info!(
        "Chunks generated: {}; avg:{:.2?} max:{:.2?} (executed:{:.2?})",
        durations.len(),
        avg,
        durations.iter().max().unwrap(),
        now.elapsed(),
    );
}

fn generate_chunk(world_generator: &WorldGenerator, chunk_position: &ChunkPosition) -> SectionsData {
    let mut sections: ArrayVec<Box<ChunkDataType>, VERTICAL_SECTIONS> = Default::default();

    for y in 0..VERTICAL_SECTIONS {
        let chunk_section = world_generator.generate_chunk_data(&chunk_position, y);
        sections.push(Box::new(chunk_section));
    }

    sections.into_inner().expect("data error")
}
