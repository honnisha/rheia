use std::collections::HashMap;

use bracket_lib::random::RandomNumberGenerator;
use godot::{
    engine::{node::InternalMode, Material, MeshInstance3D},
    prelude::*,
};
use ndshape::ConstShape;

use crate::{
    mesh::mesh_generator::{generate_chunk_geometry, ChunkBordersShape, ChunkShape},
    utils::block_mesh::VoxelVisibility,
    world_generator::WorldGenerator,
};

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
                if (Vector2::new(x as real, z as real) - p2).length() < chunks_distance as f32 {
                    for y in 0_i32..2_i32 {

                        let chunk_position = &[x, y, z];
                        if !self.is_chunk_loaded(chunk_position) {

                            self.spawn_chunk(base, chunk_position);
                        }
                    }
                }
            }
        }
    }

    pub fn is_chunk_loaded(&self, chunk_position: &[i32; 3]) -> bool {
        match self.chunks.get(chunk_position) {
            Some(c) => c.loaded,
            None => false,
        }
    }

    pub fn load_chunk(&mut self, chunk_position: &[i32; 3]) {
        if self.chunks.contains_key(chunk_position) {
            return;
        }

        let mut chunk_data = [BlockInfo::new(0); 4096];
        self.world_generator
            .generate_chunk_data(&mut chunk_data, chunk_position);

        let chunk = Chunk::new(*chunk_position, chunk_data);
        self.chunks.insert(*chunk_position, chunk);
    }

    pub fn format_chunk_data_with_boundaries<'a>(
        &'a mut self,
        chunk_data: &[BlockInfo; 4096],
        chunk_position: &[i32; 3],
    ) -> (&'a mut Self, [BlockInfo; 5832]) {
        let mut b_chunk = [BlockInfo::new(0); 5832];

        let mut has_any_mesh = false;

        for x in 0_u32..16_u32 {
            for y in 0_u32..16_u32 {
                for z in 0_u32..16_u32 {
                    let i = ChunkShape::linearize([x, y, z]);
                    assert!(
                        i < ChunkShape::SIZE,
                        "Generate chunk data overflow array length; dimentions:{:?} current:{:?}",
                        ChunkShape::ARRAY,
                        [x, y, z]
                    );

                    let b_chunk_position = ChunkBordersShape::linearize([x + 1, y + 1, z + 1]);
                    let data = chunk_data[i as usize];
                    b_chunk[b_chunk_position as usize] = data;

                    if *data.get_block_type().get_voxel_visibility() != VoxelVisibility::Empty {
                        has_any_mesh = true;
                    }
                }
            }
        }

        // fill boundaries
        if has_any_mesh {
            //godot_print!("chunk:{:?}", chunk_position);

            for axis in 0_i8..3_i8 {
                for value in (-1_i32..2_i32).step_by(2) {
                    let mut pos = chunk_position.clone();

                    pos[axis as usize] += value;
                    //godot_print!("load:{:?}", pos);

                    self.load_chunk(&pos);
                    let border_chunk = &self.get_chunk(&pos);

                    for i in 0_u32..16_u32 {
                        for j in 0_u32..16_u32 {
                            let (i_v, o_v) = match value {
                                -1 => (0, 15),
                                _ => (17, 0),
                            };

                            let (pos_inside, pos_outside) = match axis {
                                0 => ([i_v, i + 1, j + 1], [o_v, i, j]),
                                1 => ([i + 1, i_v, j + 1], [i, o_v, j]),
                                _ => ([i + 1, j + 1, i_v], [i, j, o_v]),
                            };

                            let pos_i = ChunkBordersShape::linearize(pos_inside);
                            let pos_o = ChunkShape::linearize(pos_outside);
                            //godot_print!(
                            //    "pos_inside:{:?} pos_outside:{:?}",
                            //    pos_inside,
                            //    pos_outside
                            //);
                            b_chunk[pos_i as usize] =
                                border_chunk.chunk_data[pos_o as usize].clone();
                        }
                    }
                }
            }
        }

        return (self, b_chunk);
    }

    pub fn get_mesh(&self, bordered_chunk_data: &[BlockInfo; 5832]) -> Option<Gd<MeshInstance3D>> {
        let mesh = match generate_chunk_geometry(&bordered_chunk_data) {
            Some(m) => m,
            None => return None,
        };

        let mut mesh_instance = MeshInstance3D::new_alloc();
        mesh_instance.set_mesh(mesh.upcast());
        mesh_instance.create_trimesh_collision();
        Some(mesh_instance)
    }

    pub fn get_chunk(&self, chunk_position: &[i32; 3]) -> &Chunk {
        &self.chunks.get(chunk_position).unwrap()
    }

    pub fn get_chunk_data(&self, chunk_position: &[i32; 3]) -> [BlockInfo; 4096] {
        self.chunks.get(chunk_position).unwrap().chunk_data.clone()
    }

    pub fn spawn_chunk(&mut self, base: &mut Base<Node>, chunk_position: &[i32; 3]) {

        self.load_chunk(&chunk_position);
        let chunk_data = self.get_chunk_data(chunk_position);

        let bordered_chunk_data = self.format_chunk_data_with_boundaries(&chunk_data, &chunk_position).1;
        let mesh_option = self.get_mesh(&bordered_chunk_data);

        if mesh_option.is_some() {
            let mut mesh = mesh_option.unwrap();
            let material: Option<Gd<Material>> = try_load("res://assets/world/material.tres");
            match material {
                Some(m) => mesh.set_material_overlay(m),
                None => godot_error!("Material for world not found!"),
            }

            let p = chunk_position;
            mesh.set_name(GodotString::from(format!(
                "chunk_{}_{}_{}",
                p[0], p[1], p[2]
            )));
            mesh.set_position(Chunk::get_chunk_position_from_position(chunk_position));
            base.add_child(mesh.upcast(), false, InternalMode::INTERNAL_MODE_BACK);
        }

        let mut chunk = self.chunks.get_mut(chunk_position).unwrap();
        chunk.loaded = true;
    }
}

impl AsRef<ChunksManager> for ChunksManager {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl AsMut<ChunksManager> for ChunksManager {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}
