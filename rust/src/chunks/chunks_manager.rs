use std::{collections::HashMap, borrow::BorrowMut, ops::DerefMut};

use godot::{engine::{node::InternalMode, Material}, prelude::*};

use super::chunk::Chunk;

pub struct ChunksManager {
    chunks: HashMap<[i32; 3], Chunk>,
    material: Option<Gd<Material>>,
}

impl ChunksManager {
    pub fn new() -> Self {
        let material: Option<Gd<Material>> = try_load("res://assets/world/material.tres");

        if material.is_none() {
            godot_error!("Material for world not found!");
        }

        ChunksManager {
            chunks: HashMap::new(),
            material: material,
        }
    }

    #[allow(unused_variables)]
    pub fn update_camera_position(&mut self, base: &mut Base<Node>, camera_position: Vector3) {
        self.load_chunk(base, &[0_i32, 0_i32, 0_i32]);
        self.load_chunk(base, &[1_i32, 0_i32, 0_i32]);
        self.load_chunk(base, &[2_i32, 0_i32, 0_i32]);
    }

    pub fn load_chunk(&mut self, base: &mut Base<Node>, chunk_position: &[i32; 3]) {
        if !self.chunks.contains_key(chunk_position) {
            let mut chunk = Chunk::new(*chunk_position);
            let mut mesh = chunk.get_mesh();

            let material: Gd<Material> = try_load("res://assets/world/material.tres").unwrap();
            mesh.set_material_overlay(material);

            base.add_child(mesh.upcast(), false, InternalMode::INTERNAL_MODE_BACK);
            godot_print!("Chunk {:?} loaded", chunk.get_position());
            self.chunks.insert(*chunk_position, chunk);
        }
    }
}
