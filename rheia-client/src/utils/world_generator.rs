use common::{
    chunks::{chunk_data::ChunkData, chunk_position::ChunkPosition},
    utils::spiral_iterator::SpiralIterator,
    world_generator::{
        default::{WorldGenerator, WorldGeneratorSettings},
        traits::IWorldGenerator,
    },
};
use godot::obj::Gd;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use std::collections::HashMap;

use crate::world::world_manager::WorldManager;

pub fn generate_chunks(
    world: &mut Gd<WorldManager>,
    x: i32,
    z: i32,
    chunks_distance: i32,
    settings: WorldGeneratorSettings,
) {
    let world_generator = WorldGenerator::create(None, settings).unwrap();

    let mut chunks: HashMap<ChunkPosition, Option<ChunkData>> = Default::default();
    let iter = SpiralIterator::new(x as i64, z as i64, chunks_distance as i64);
    for (x, z) in iter {
        let chunk_pos = ChunkPosition::new(x, z);
        chunks.insert(chunk_pos, None);
    }

    // Generate chunks using rayon paralelism
    chunks.par_iter_mut().for_each(|(chunk_pos, data)| {
        let sections = world_generator.generate_chunk_data(&chunk_pos);
        *data = Some(sections);
    });

    let center = ChunkPosition::zero();
    for (chunk_pos, data) in chunks {
        world.bind_mut().recieve_chunk(center, chunk_pos, data.unwrap());
    }
}
