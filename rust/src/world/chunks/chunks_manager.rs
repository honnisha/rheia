use bracket_lib::random::RandomNumberGenerator;
use godot::{
    engine::{node::InternalMode, Material},
    prelude::*,
};
use ndshape::ConstShape;
use send_wrapper::SendWrapper;
use std::time::Instant;
use std::{
    collections::HashMap,
    ops::DerefMut,
    sync::mpsc::{self, Receiver, Sender},
};

use crate::{
    utils::mesh::block_mesh::VoxelVisibility,
    utils::mesh::mesh_generator::{ChunkBordersShape, ChunkShape},
    utils::{
        mesh::mesh_generator::{generate_chunk_geometry, Geometry},
        textures::{material_builder::build_blocks_material, texture_mapper::TextureMapper},
    },
    world::blocks::blocks_storage::BlockType,
    world::world_generator::WorldGenerator,
};

use super::{block_info::BlockInfo, chunk::Chunk};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct ChunksManager {
    #[base]
    base: Base<Node>,

    chunks_ids: HashMap<[i32; 3], i64>,
    world_generator: WorldGenerator,

    texture_mapper: TextureMapper,
    material: Gd<Material>,

    tx: Sender<(SendWrapper<Gd<Chunk>>, Geometry)>,
    rx: Receiver<(SendWrapper<Gd<Chunk>>, Geometry)>,
}

#[godot_api]
impl ChunksManager {}

impl ChunksManager {
    #[allow(unused_variables)]
    pub fn update_camera_position(&mut self, base: &mut Base<Node>, camera_position: Vector3) {
        let now = Instant::now();
        let mut count: i32 = 0;
        let chunks_distance = 12;

        let chunk_x = ((camera_position.x as f32) / 16_f32) as i32;
        let chunk_z = ((camera_position.z as f32) / 16_f32) as i32;

        let p2 = Vector2::new(chunk_x as real, chunk_z as real);

        for x in (chunk_x - chunks_distance)..(chunk_x + chunks_distance) {
            for z in (chunk_z - chunks_distance)..(chunk_z + chunks_distance) {
                if (Vector2::new(x as real, z as real) - p2).length() < chunks_distance as f32 {
                    for y in 0_i32..16_i32 {
                        let chunk_position = &[x, y, z];
                        if !self.is_chunk_loaded(chunk_position) {
                            self.spawn_chunk(chunk_position);
                            count += 1;
                        }
                    }
                }
            }
        }
        if count > 0 {
            println!(
                "update_camera_position complete {:.2?}; chunks: {}",
                now.elapsed(),
                count
            );
        }
    }

    pub fn modify_block(&mut self, pos: &[i32; 3], block_info: BlockInfo) {
        let chunk_pos = Chunk::get_chunk_pos_by_global(pos);
        if let Some(mut c) = self.get_chunk(&chunk_pos) {
            c.bind_mut().set_block(pos, block_info);

            self.update_chunk_mesh(&mut c);
        }
    }

    pub fn modify_block_batch(&mut self, data: HashMap<[i32; 3], HashMap<u32, BlockInfo>>) -> i32 {
        let now = Instant::now();
        println!("modify_block_batch: Start to update {} blocks", data.len());

        let mut updated_chunks: Vec<i64> = Vec::new();
        let mut count: i32 = 0;

        for (chunk_pos, chunk_data) in data {
            if let Some(mut c) = self.get_chunk(&chunk_pos) {
                for (block_local_pos, block_info) in chunk_data {
                    c.bind_mut().set_block_by_local_pos(block_local_pos, block_info);
                    count += 1;
                }
                updated_chunks.push(c.bind().get_index(true));
            } else {
                //println!("modify_block_batch: Chunk {:?} not found", chunk_pos);
            }
        }

        for updated_chunk in updated_chunks {
            let mut c = self.get_chunk_by_index(updated_chunk).unwrap();
            self.update_chunk_mesh(&mut c);
            //println!("update chunk mesh:{:?}", c);
        }
        println!("modify_block_batch: Update complete in {:.2?}", now.elapsed());
        count
    }

    fn update_chunk_mesh(&mut self, chunk: &mut Gd<Chunk>) {
        let bordered_chunk_data: [BlockType; 5832];
        {
            let chunk_ref = chunk.bind();
            let chunk_data = chunk_ref.get_chunk_data();
            let chunk_position = chunk_ref.get_chunk_position();
            bordered_chunk_data = self.format_chunk_data_with_boundaries(&chunk_data, &chunk_position);
        }

        // Copy transmitter
        let tx = self.tx.clone();

        //let chunk_position = chunk.bind().get_chunk_position().clone();
        let wrapped_chunk = SendWrapper::new(chunk.share());
        let texture_mapper = self.texture_mapper.clone();
        rayon::spawn(move || {
            let new_geometry = generate_chunk_geometry(&texture_mapper, &bordered_chunk_data);
            tx.send((wrapped_chunk, new_geometry)).unwrap();
        });
    }

    pub fn get_chunk_by_index(&self, index: i64) -> Option<Gd<Chunk>> {
        if let Some(n) = self.base.get_child(index, true) {
            return Some(n.cast::<Chunk>());
        }
        return None;
    }

    pub fn get_chunk(&self, chunk_position: &[i32; 3]) -> Option<Gd<Chunk>> {
        if let Some(index) = self.chunks_ids.get(chunk_position) {
            return self.get_chunk_by_index(*index);
        }
        return None;
    }

    pub fn is_chunk_loaded(&self, chunk_position: &[i32; 3]) -> bool {
        match self.get_chunk(chunk_position) {
            Some(c) => c.bind().is_loaded(),
            None => false,
        }
    }

    pub fn load_chunk(&mut self, chunk_position: &[i32; 3]) -> i64 {
        {
            if let Some(index) = self.chunks_ids.get(chunk_position) {
                return index.clone();
            }
        }

        let mut chunk_data = [BlockInfo::new(BlockType::Air); 4096];
        self.world_generator
            .generate_chunk_data(&mut chunk_data, chunk_position);

        let mut chunk = Gd::<Chunk>::with_base(|base| Chunk::create(base, chunk_data, chunk_position.clone()));

        let chunk_name = GodotString::from(format!(
            "chunk_{}_{}_{}",
            chunk_position[0], chunk_position[1], chunk_position[2]
        ));
        chunk.bind_mut().base.set_name(chunk_name.clone());

        let global_pos = Chunk::get_chunk_position_from_coordinate(&chunk_position);

        self.base
            .add_child(chunk.upcast(), true, InternalMode::INTERNAL_MODE_FRONT);

        let mut c = self.base.get_node_as::<Node3D>(&chunk_name);
        let index = c.get_index(true);
        self.chunks_ids.insert(*chunk_position, index.clone());

        c.set_global_position(global_pos);
        c.cast::<Chunk>().bind_mut().create_mesh(&self.material);
        index.clone()
    }

    pub fn format_chunk_data_with_boundaries(
        &mut self,
        chunk_data: &[BlockInfo; 4096],
        chunk_position: &[i32; 3],
    ) -> [BlockType; 5832] {
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
                            b_chunk[pos_i as usize] = border_chunk.as_ref().unwrap().bind().get_chunk_data()
                                [pos_o as usize]
                                .get_block_type()
                                .clone();
                        }
                    }
                }
            }
        }

        return b_chunk;
    }

    pub fn spawn_chunk(&mut self, chunk_position: &[i32; 3]) {
        let index = self.load_chunk(&chunk_position);
        if let Some(mut chunk) = self.get_chunk_by_index(index) {
            self.update_chunk_mesh(&mut chunk);
        } else {
            godot_error!(
                "Cant find chunk to spawn at {:?} by index:{} childs count:{}",
                chunk_position,
                index,
                self.base.get_child_count(true)
            );
        }
    }
}

#[godot_api]
impl NodeVirtual for ChunksManager {
    fn init(base: Base<Node>) -> Self {
        let mut rng = RandomNumberGenerator::new();
        let seed = rng.next_u64();
        let mut texture_mapper = TextureMapper::new();

        let (tx, rx) = mpsc::channel();

        let texture = build_blocks_material(&mut texture_mapper);
        ChunksManager {
            base,
            chunks_ids: HashMap::new(),
            world_generator: WorldGenerator::new(seed),
            material: texture.duplicate(true).unwrap().cast::<Material>(),
            texture_mapper: texture_mapper,
            tx: tx,
            rx: rx,
        }
    }

    #[allow(unused_variables)]
    fn process(&mut self, delta: f64) {
        for (mut wrapped_chunk, new_geometry) in self.rx.try_iter() {
            wrapped_chunk.deref_mut().bind_mut().update_mesh(new_geometry.mesh_ist);
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
