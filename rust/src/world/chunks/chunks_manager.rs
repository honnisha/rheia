use std::collections::HashMap;

use bracket_lib::random::RandomNumberGenerator;
use godot::{
    engine::{node::InternalMode, StandardMaterial3D},
    prelude::*,
};
use ndshape::ConstShape;

use crate::{
    utils::mesh::block_mesh::VoxelVisibility,
    utils::mesh::mesh_generator::{ChunkBordersShape, ChunkShape},
    utils::textures::{material_builder::build_blocks_material, texture_mapper::TextureMapper},
    world::blocks::blocks_storage::BlockType,
    world::world_generator::WorldGenerator,
};

use super::{block_info::BlockInfo, chunk::Chunk};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct ChunksManager {
    #[base]
    base: Base<Node>,

    chunks: HashMap<[i32; 3], Gd<Chunk>>,
    world_generator: WorldGenerator,

    texture_mapper: TextureMapper,
    material: Gd<StandardMaterial3D>,
}

#[godot_api]
impl ChunksManager {}

impl ChunksManager {
    pub fn modify_block(&mut self, pos: &[i32; 3], block_type: BlockType) {
        let chunk_pos = Chunk::get_chunk_positions_by_coordinate(pos);
        let c = self.chunks.get_mut(&chunk_pos);

        if c.is_some() {
            c.unwrap().bind_mut().set_block(pos, block_type);
        }
    }

    #[allow(unused_variables)]
    pub fn update_camera_position(&mut self, base: &mut Base<Node>, camera_position: Vector3) {
        let chunks_distance = 3;

        let chunk_x = ((camera_position.x as f32) / 16_f32) as i32;
        let chunk_z = ((camera_position.z as f32) / 16_f32) as i32;

        let p2 = Vector2::new(chunk_x as real, chunk_z as real);

        for x in (chunk_x - chunks_distance)..(chunk_x + chunks_distance) {
            for z in (chunk_z - chunks_distance)..(chunk_z + chunks_distance) {
                if (Vector2::new(x as real, z as real) - p2).length() < chunks_distance as f32 {
                    for y in 0_i32..2_i32 {
                        let chunk_position = &[x, y, z];
                        if !self.is_chunk_loaded(chunk_position) {
                            self.spawn_chunk(chunk_position);
                        }
                    }
                }
            }
        }
    }

    pub fn is_chunk_loaded(&self, chunk_position: &[i32; 3]) -> bool {
        match self.chunks.get(chunk_position) {
            Some(c) => c.bind().is_loaded(),
            None => false,
        }
    }

    pub fn load_chunk(&mut self, chunk_position: &[i32; 3]) {
        if self.chunks.contains_key(chunk_position) {
            return;
        }

        let mut chunk_data = [BlockInfo::new(BlockType::Air); 4096];
        self.world_generator
            .generate_chunk_data(&mut chunk_data, chunk_position);

        let mut chunk = Gd::<Chunk>::with_base(|base| Chunk::create(base, chunk_data));
        chunk
            .bind_mut()
            .create_mesh(&chunk_position, &self.material);

        let chunk_name = GodotString::from(format!(
            "chunk_{}_{}_{}",
            chunk_position[0], chunk_position[1], chunk_position[2]
        ));
        chunk.bind_mut().base.set_name(chunk_name.clone());

        self.base
            .add_child(chunk.upcast(), true, InternalMode::INTERNAL_MODE_BACK);
        let c = self.base.get_node_as::<Chunk>(&chunk_name);
        self.chunks.insert(*chunk_position, c.cast::<Chunk>());
    }

    pub fn format_chunk_data_with_boundaries<'a>(
        &'a mut self,
        chunk_data: &[BlockInfo; 4096],
        chunk_position: &[i32; 3],
    ) -> (&'a mut Self, [BlockType; 5832]) {
        let mut b_chunk = [BlockType::Air; 5832];

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
                    b_chunk[b_chunk_position as usize] = data.get_block_type().clone();

                    if *data
                        .get_block_type()
                        .get_block_type_info()
                        .unwrap()
                        .get_voxel_visibility()
                        != VoxelVisibility::Empty
                    {
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
                            b_chunk[pos_i as usize] = border_chunk.bind().get_chunk_data()
                                [pos_o as usize]
                                .get_block_type()
                                .clone();
                        }
                    }
                }
            }
        }

        return (self, b_chunk);
    }

    pub fn get_chunk(&self, chunk_position: &[i32; 3]) -> &Gd<Chunk> {
        &self.chunks.get(chunk_position).unwrap()
    }

    pub fn spawn_chunk(&mut self, chunk_position: &[i32; 3]) {
        self.load_chunk(&chunk_position);

        let chunk_data: [BlockInfo; 4096];
        let cs = &mut self.chunks;
        {
            chunk_data = cs.get_mut(chunk_position).unwrap().bind().get_chunk_data().clone();
        }

        let bordered_chunk_data = self.format_chunk_data_with_boundaries(&chunk_data, &chunk_position).1;

        let chunk = self.chunks.get_mut(chunk_position).unwrap();
        chunk
            .bind_mut()
            .update_mesh(&bordered_chunk_data, &self.texture_mapper);
    }
}

#[godot_api]
impl NodeVirtual for ChunksManager {
    fn init(base: Base<Node>) -> Self {
        let mut rng = RandomNumberGenerator::new();
        let seed = rng.next_u64();
        let mut texture_mapper = TextureMapper::new();

        ChunksManager {
            base,
            chunks: HashMap::new(),
            world_generator: WorldGenerator::new(seed),
            material: build_blocks_material(&mut texture_mapper),
            texture_mapper: texture_mapper,
        }
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
