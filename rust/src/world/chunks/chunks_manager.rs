use bracket_lib::random::RandomNumberGenerator;
use godot::{
    engine::{node::InternalMode, Material},
    prelude::*,
};
use std::{
    collections::HashMap,
    sync::{
        mpsc::{self, Receiver, Sender},
        Mutex, MutexGuard,
    },
};
use std::{
    sync::{Arc, RwLock},
    time::Instant,
};

use crate::{
    utils::{
        mesh::mesh_generator::{generate_chunk_geometry, Geometry},
        textures::{material_builder::build_blocks_material, texture_mapper::TextureMapper},
    },
    world::blocks::blocks_storage::BlockType,
    world::world_generator::WorldGenerator,
};

use super::{
    block_info::BlockInfo, chunk::Chunk, chunk_data_formatter::format_chunk_data_with_boundaries, chunk_info::ChunkInfo,
};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct ChunksManager {
    #[base]
    base: Base<Node>,

    chunks_info: Arc<Mutex<HashMap<[i32; 3], ChunkInfo>>>,
    world_generator: Arc<RwLock<WorldGenerator>>,

    texture_mapper: Arc<RwLock<TextureMapper>>,
    material: Gd<Material>,

    update_mesh_tx: Sender<(i64, Geometry)>,
    update_mesh_rx: Receiver<(i64, Geometry)>,
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

        let chunk_pos = Vector2::new(chunk_x as real, chunk_z as real);

        for x in (chunk_x - chunks_distance)..(chunk_x + chunks_distance) {
            for z in (chunk_z - chunks_distance)..(chunk_z + chunks_distance) {
                if (Vector2::new(x as real, z as real) - chunk_pos).length() < chunks_distance as f32 {
                    for y in 0_i32..16_i32 {
                        let chunk_position = &[x, y, z];

                        if !self.is_chunk_loaded(chunk_position) {
                            let chunks_info = self.chunks_info.clone();
                            let mut ci = match chunks_info.lock() {
                                Ok(c) => c,
                                Err(e) => {
                                    println!("update_camera_position; pos: {:?} lock error: {:?}", chunk_position, e);
                                    continue;
                                }
                            };

                            {
                                match ChunksManager::get_or_load_chunk_data(
                                    self.world_generator.clone(),
                                    &mut ci,
                                    chunk_position,
                                ) {
                                    Some(e) => e,
                                    _ => continue,
                                };
                            }

                            let chunk = self.spawn_chunk(chunk_position);
                            let index = chunk.bind().get_index(true).clone();

                            let chunk_info = ci.get_mut(chunk_position).unwrap();
                            chunk_info.set_godot_index(index);

                            {
                                ChunksManager::update_chunk_mesh(
                                    &mut ci,
                                    chunk_position.clone(),
                                    self.world_generator.clone(),
                                    self.update_mesh_tx.clone(),
                                    self.texture_mapper.clone(),
                                );
                            }

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

    pub fn modify_block(&self, global_pos: &[i32; 3], block_info: BlockInfo) {
        let chunk_pos = ChunkInfo::get_chunk_pos_by_global(global_pos);

        let mut ci = match self.chunks_info.lock() {
            Ok(c) => c,
            Err(e) => {
                println!("modify_block_batch; lock error: {:?}", e);
                return;
            }
        };

        let info = if let Some(info) = ci.get_mut(&chunk_pos) {
            info
        } else {
            println!("modify_block: Cant find ChunkInfo in {:?}", chunk_pos);
            return;
        };

        info.set_block(global_pos, block_info);
        ChunksManager::update_chunk_mesh(
            &mut ci,
            chunk_pos.clone(),
            self.world_generator.clone(),
            self.update_mesh_tx.clone(),
            self.texture_mapper.clone(),
        );
    }

    pub fn modify_block_batch(&mut self, data: HashMap<[i32; 3], HashMap<u32, BlockInfo>>) -> i32 {
        let now = Instant::now();
        println!("modify_block_batch: Start to update {} blocks", data.len());

        let mut ci = match self.chunks_info.lock() {
            Ok(c) => c,
            Err(e) => {
                println!("modify_block_batch; lock error: {:?}", e);
                return 0_i32;
            }
        };

        let mut count: i32 = 0;
        let mut chunks_pos: Vec<[i32; 3]> = Vec::new();

        for (chunk_pos, chunk_data) in data {
            if let Some(info) = ci.get_mut(&chunk_pos) {
                for (block_local_pos, block_info) in chunk_data {
                    info.set_block_by_local_pos(block_local_pos, block_info);
                    count += 1;
                }
                chunks_pos.push(chunk_pos);
            } else {
                println!("modify_block_batch: Cant find ChunkInfo in {:?}", chunk_pos);
            }
        }

        for chunk_pos in chunks_pos {
            ChunksManager::update_chunk_mesh(
                &mut ci,
                chunk_pos.clone(),
                self.world_generator.clone(),
                self.update_mesh_tx.clone(),
                self.texture_mapper.clone(),
            );
            //println!("update chunk mesh:{:?}", c);
        }
        println!("modify_block_batch: Update complete in {:.2?}", now.elapsed());
        count
    }

    fn update_chunk_mesh(
        ci: &mut MutexGuard<HashMap<[i32; 3], ChunkInfo>>,
        chunk_pos: [i32; 3],
        world_generator: Arc<RwLock<WorldGenerator>>,
        update_mesh_tx: Sender<(i64, Geometry)>,
        texture_mapper: Arc<RwLock<TextureMapper>>,
    ) {
        let info = match ci.get(&chunk_pos) {
            Some(e) => e,
            _ => {
                println!("update_chunk_mesh error: no ChunkInfo in {:?}", chunk_pos);
                return;
            }
        };
        let index = match info.get_godot_index() {
            Some(i) => i,
            _ => {
                println!("update_chunk_mesh error: no index in {:?}", chunk_pos);
                return;
            }
        };
        let chunk_data = info.get_chunk_data().clone();

        let bordered_chunk_data =
            format_chunk_data_with_boundaries(world_generator.clone(), ci, &chunk_data, &chunk_pos);

        rayon::spawn(move || {
            let new_geometry = generate_chunk_geometry(texture_mapper, &bordered_chunk_data);
            update_mesh_tx.send((index.clone(), new_geometry)).unwrap();
        });
    }

    pub fn get_chunk_by_index(&self, index: i64) -> Option<Gd<Chunk>> {
        if let Some(n) = self.base.get_child(index, true) {
            return Some(n.cast::<Chunk>());
        }
        return None;
    }

    pub fn is_chunk_loaded(
        ci: &'a mut MutexGuard<HashMap<[i32; 3], ChunkInfo>>,
        &self, chunk_position: &[i32; 3]
    ) -> bool {
        let ci = match self.chunks_info.lock() {
            Ok(c) => c,
            Err(e) => {
                println!("update_chunk_mesh; lock error: {:?}", e);
                return false;
            }
        };
        if let Some(info) = ci.get(chunk_position) {
            return info.is_loaded();
        }
        return false;
    }

    pub fn get_or_load_chunk_data<'a>(
        world_generator: Arc<RwLock<WorldGenerator>>,
        ci: &'a mut MutexGuard<HashMap<[i32; 3], ChunkInfo>>,
        chunk_position: &[i32; 3],
    ) -> Option<&'a mut ChunkInfo> {
        if !ci.contains_key(chunk_position) {
            let mut chunk_data = [BlockInfo::new(BlockType::Air); 4096];
            world_generator
                .read()
                .unwrap()
                .generate_chunk_data(&mut chunk_data, chunk_position);

            ci.insert(*chunk_position, ChunkInfo::new(chunk_data));
        }

        Some(ci.get_mut(chunk_position).unwrap())
    }

    pub fn spawn_chunk(&mut self, chunk_position: &[i32; 3]) -> Gd<Chunk> {
        let mut chunk = Gd::<Chunk>::with_base(|base| Chunk::create(base));

        let chunk_name = GodotString::from(format!(
            "chunk_{}_{}_{}",
            chunk_position[0], chunk_position[1], chunk_position[2]
        ));
        chunk.bind_mut().base.set_name(chunk_name.clone());

        let global_pos = ChunkInfo::get_chunk_position_from_coordinate(&chunk_position);

        self.base
            .add_child(chunk.upcast(), true, InternalMode::INTERNAL_MODE_FRONT);

        let mut c = self.base.get_node_as::<Node3D>(&chunk_name);

        c.set_global_position(global_pos);
        let mut ch = c.cast::<Chunk>();
        ch.bind_mut().create_mesh(&self.material);
        ch
    }
}

#[godot_api]
impl NodeVirtual for ChunksManager {
    fn init(base: Base<Node>) -> Self {
        let mut rng = RandomNumberGenerator::new();
        let seed = rng.next_u64();
        let mut texture_mapper = TextureMapper::new();

        let (update_mesh_tx, update_mesh_rx) = mpsc::channel();

        let texture = build_blocks_material(&mut texture_mapper);
        ChunksManager {
            base,
            chunks_info: Arc::new(Mutex::new(HashMap::new())),
            world_generator: Arc::new(RwLock::new(WorldGenerator::new(seed))),
            material: texture.duplicate(true).unwrap().cast::<Material>(),
            texture_mapper: Arc::new(RwLock::new(texture_mapper)),
            update_mesh_tx: update_mesh_tx,
            update_mesh_rx: update_mesh_rx,
        }
    }

    #[allow(unused_variables)]
    fn process(&mut self, delta: f64) {
        for (chunk_index, new_geometry) in self.update_mesh_rx.try_iter() {
            if let Some(mut c) = self.get_chunk_by_index(chunk_index) {
                // println!("Mesh updated: {:?}; surfaces: {}", chunk_pos, new_geometry.mesh_ist.get_surface_count());
                c.bind_mut().update_mesh(new_geometry.mesh_ist);
            } else {
                println!("Cant update mesh for chunk: index {:?} not found", chunk_index);
            }
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
