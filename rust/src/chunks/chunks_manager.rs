use std::collections::HashMap;

use bracket_lib::random::RandomNumberGenerator;
use godot::{
    engine::{node::InternalMode, Material},
    prelude::*,
};

use crate::world_generator::WorldGenerator;

use super::{block_info::BlockInfo, chunk::Chunk};

pub struct ChunksManager {
    chunks: HashMap<[i32; 3], Chunk>,
    world_generator: WorldGenerator,
}

impl ChunksManager {
    pub fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();

        ChunksManager {
            chunks: HashMap::new(),
            world_generator: WorldGenerator::new(rng.next_u64()),
        }
    }

    #[allow(unused_variables)]
    pub fn update_camera_position(&mut self, base: &mut Base<Node>, camera_position: Vector3) {
        let chunks_distance = 1;

        let chunk_x = ((camera_position.x as f32) / 16_f32) as i32;
        let chunk_z = ((camera_position.z as f32) / 16_f32) as i32;

        let p2 = Vector2::new(chunk_x as real, chunk_z as real);

        for x in (chunk_x - chunks_distance)..(chunk_x + chunks_distance) {
            for z in (chunk_z - chunks_distance)..(chunk_z + chunks_distance) {
                if (Vector2::new(x as real, z as real) - p2).length() <= chunks_distance as f32 {

                    for y in 0_i32..4_i32 {
                        self.load_chunk(base, &[x, y, z]);
                    }
                }
            }
        }
    }

    pub fn get_chunk_data(&mut self, chunk_position: &[i32; 3]) -> [BlockInfo; 4096] {
        let mut chunk_data = [BlockInfo::new(0); 4096];
        self.world_generator
            .generate_chunk_data(&mut chunk_data, chunk_position);
        return chunk_data;
    }

    pub fn load_chunk(&mut self, base: &mut Base<Node>, chunk_position: &[i32; 3]) {
        if !self.chunks.contains_key(chunk_position) {
            let mut chunk = Chunk::new(*chunk_position);

            chunk.set_chunk_data(self.get_chunk_data(&chunk_position));

            let mesh_option = chunk.get_mesh();
            if mesh_option.is_some() {
                let mut mesh = mesh_option.unwrap();
                let material: Option<Gd<Material>> = try_load("res://assets/world/material.tres");
                match material {
                    Some(m) => mesh.set_material_overlay(m),
                    None => godot_error!("Material for world not found!"),
                }

                let p = chunk.get_position();
                mesh.set_name(GodotString::from(format!("chunk_{}_{}_{}", p[0], p[1], p[2])));
                base.add_child(mesh.upcast(), false, InternalMode::INTERNAL_MODE_BACK);
            }

            self.chunks.insert(*chunk_position, chunk);
        }
    }
}
