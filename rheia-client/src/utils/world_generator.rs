use std::collections::HashMap;

use arrayvec::ArrayVec;
use common::{
    chunks::chunk_position::ChunkPosition,
    world_generator::default::{WorldGenerator, WorldGeneratorSettings},
    VERTICAL_SECTIONS,
};
use godot::obj::Gd;
use network::messages::{ChunkDataType, SectionsData};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use spiral::ManhattanIterator;

use crate::world::world_manager::WorldManager;

pub fn generate_chunks(
    world: &mut Gd<WorldManager>,
    x: i32,
    z: i32,
    chunks_distance: u16,
    settings: WorldGeneratorSettings,
) {
    let now = std::time::Instant::now();

    let world_generator = WorldGenerator::create(None, settings).unwrap();

    let mut chunks: HashMap<ChunkPosition, Option<SectionsData>> = Default::default();
    let iter = ManhattanIterator::new(x, z, chunks_distance);
    for (x, z) in iter {
        let chunk_pos = ChunkPosition::new(x as i64, z as i64);
        chunks.insert(chunk_pos, None);
    }

    // Generate chunks using rayon paralelism
    chunks.par_iter_mut().for_each(|(chunk_pos, data)| {
        *data = Some(generate_chunk(&world_generator, &chunk_pos));
    });

    for (chunk_pos, data) in chunks {
        world.bind_mut().recieve_chunk(chunk_pos, data.unwrap());
    }

    log::info!(target: "world_generator", "Chunks generated (executed:{:.2?})", now.elapsed());
}

fn generate_chunk(world_generator: &WorldGenerator, chunk_position: &ChunkPosition) -> SectionsData {
    let mut sections: ArrayVec<Box<ChunkDataType>, VERTICAL_SECTIONS> = Default::default();

    for y in 0..VERTICAL_SECTIONS {
        let chunk_section = world_generator.generate_chunk_data(&chunk_position, y);
        sections.push(Box::new(chunk_section));
    }

    sections.into_inner().expect("data error")
}
