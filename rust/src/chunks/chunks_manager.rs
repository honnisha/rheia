use std::collections::HashMap;

use godot::{engine::node::InternalMode, prelude::*};

use super::chunk::Chunk;

pub struct ChunksManager {
    chunks: HashMap<[i32; 3], Chunk>,
}

impl ChunksManager {
    pub fn new() -> Self {
        ChunksManager {
            chunks: HashMap::new(),
        }
    }

    #[allow(unused_variables)]
    pub fn update_camera_position(&mut self, base: &mut Base<Node>, camera_position: Vector3) {
        let chunk_position = [0, 0, 0];
        if !self.chunks.contains_key(&chunk_position) {
            let mut chunk = Chunk::new(chunk_position);
            let mesh = chunk.get_mesh();
            base.add_child(mesh.upcast(), false, InternalMode::INTERNAL_MODE_BACK);
            godot_print!("Chunk {:?} loaded", chunk.get_position());
            self.chunks.insert(chunk_position, chunk);
        }
    }
}
