use bracket_lib::random::RandomNumberGenerator;
use godot::{
    engine::{node::InternalMode, Material},
    prelude::*,
};
use ndshape::ConstShape;
use spiral::ManhattanIterator;
use std::sync::{Arc, RwLock};
use std::{
    collections::HashMap,
    sync::{
        mpsc::{self, Receiver, Sender},
        RwLockReadGuard, RwLockWriteGuard,
    },
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
    block_info::BlockInfo,
    chunk::Chunk,
    chunk_data_formatter::{format_chunk_data_with_boundaries, get_boundaries_chunks},
    chunk_info::{ChunkInfo, ChunkShape, CHUNK_SIZE},
};

pub type ChunksInfoType = Arc<RwLock<HashMap<[i32; 3], ChunkInfo>>>;
pub type ChunksInfoLockRead<'a> = RwLockReadGuard<'a, HashMap<[i32; 3], ChunkInfo>>;
pub type ChunksInfoLockWrite<'a> = RwLockWriteGuard<'a, HashMap<[i32; 3], ChunkInfo>>;

pub const WORLD_CHUNKS_FROM: i32 = 0_i32;
pub const WORLD_CHUNKS_TO: i32 = 16_i32;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct ChunksManager {
    #[base]
    base: Base<Node>,

    chunks_info: ChunksInfoType,
    chunks_godot_ids: HashMap<[i32; 3], i64>,
    world_generator: Arc<RwLock<WorldGenerator>>,

    texture_mapper: Arc<RwLock<TextureMapper>>,
    material: Gd<Material>,

    update_mesh_tx: Sender<([i32; 3], Geometry)>,
    update_mesh_rx: Receiver<([i32; 3], Geometry)>,
}

#[godot_api]
impl ChunksManager {}

impl ChunksManager {
    #[allow(unused_variables)]
    pub fn update_camera_position(&mut self, base: &mut Base<Node>, camera_position: Vector3) {
        let chunks_distance = 12;

        let chunk_x = ((camera_position.x as f32) / (CHUNK_SIZE as f32)) as i32;
        let chunk_z = ((camera_position.z as f32) / (CHUNK_SIZE as f32)) as i32;

        let chunk_pos = Vector2::new(chunk_x as real, chunk_z as real);

        let mut chunks_to_load = Vec::new();

        let iter = ManhattanIterator::new(chunk_x, chunk_z, chunks_distance);
        for (x, z) in iter {
            for y in WORLD_CHUNKS_FROM..WORLD_CHUNKS_TO {
                let chunk_pos = [x, y, z].clone();

                if self.chunks_godot_ids.contains_key(&chunk_pos) {
                    continue;
                }

                let chunk = self.spawn_chunk(&chunk_pos);
                let index = chunk.bind().get_index(true).clone();

                self.chunks_godot_ids.insert(chunk_pos.clone(), index);
                //println!("Chunk object spawned: {:?} index {}", chunk_pos, index);

                if !self.is_loaded(&chunk_pos) {
                    chunks_to_load.push(chunk_pos);
                }
            }
        }

        let chunks_info = self.chunks_info.clone();
        let world_generator = self.world_generator.clone();
        let update_mesh_tx = self.update_mesh_tx.clone();
        let texture_mapper = self.texture_mapper.clone();
        rayon::spawn(move || {
            for chunk_pos in chunks_to_load {
                {
                    let c = chunks_info.clone();

                    let mut ci_write = match c.write() {
                        Ok(l) => l,
                        Err(e) => {
                            println!("UPDATE_CAMERA_POSITION excepts lock; error: {:?}", e);
                            return;
                        }
                    };
                    let has_any_block =
                        ChunksManager::load_chunk_data(world_generator.clone(), &mut ci_write, &chunk_pos);

                    if has_any_block {
                        // Load chunks in border
                        let boundary = get_boundaries_chunks(&chunk_pos);
                        for (_axis, _value, pos) in boundary {
                            if pos[1] < WORLD_CHUNKS_TO && pos[1] >= WORLD_CHUNKS_FROM {
                                if !ci_write.contains_key(&pos) {
                                    ChunksManager::load_chunk_data(world_generator.clone(), &mut ci_write, &pos);
                                }
                            }
                        }
                    }
                    else {
                        continue;
                    }
                    // println!("Chunk loaded {:?}", chunk_pos);
                }

                let ci = chunks_info.clone();
                let tx = update_mesh_tx.clone();
                let tm = texture_mapper.clone();
                rayon::spawn(move || {
                    ChunksManager::update_chunk_mesh(ci, chunk_pos.clone(), tx, tm);
                    // println!("Update mesh {:?}", chunk_pos);
                });
            }
        });
    }

    pub fn modify_block(&self, global_pos: &[i32; 3], block_info: BlockInfo) {
        let chunk_pos = ChunkInfo::get_chunk_pos_by_global(global_pos);

        let mut ci = self.chunks_info.write().expect("MODIFY_BLOCK_BATCH excepts lock");

        let info = if let Some(info) = ci.get_mut(&chunk_pos) {
            info
        } else {
            println!("MODIFY_BLOCK: Cant find ChunkInfo in {:?}", chunk_pos);
            return;
        };

        info.set_block(global_pos, block_info);
        ChunksManager::update_chunk_mesh(
            self.chunks_info.clone(),
            chunk_pos.clone(),
            self.update_mesh_tx.clone(),
            self.texture_mapper.clone(),
        );
    }

    pub fn modify_block_batch(&mut self, data: HashMap<[i32; 3], HashMap<u32, BlockInfo>>) {
        println!("modify_block_batch: Start to update {} blocks", data.len());

        let chunks_info = self.chunks_info.clone();
        let update_mesh_tx = self.update_mesh_tx.clone();
        let texture_mapper = self.texture_mapper.clone();

        //rayon::spawn(move || {
            let c = chunks_info.clone();
            let mut ci = c.write().expect("MODIFY_BLOCK_BATCH excepts lock");

            let mut chunks_pos: Vec<[i32; 3]> = Vec::new();

            for (chunk_pos, chunk_data) in data {
                if let Some(info) = ci.get_mut(&chunk_pos) {
                    for (block_local_pos, block_info) in chunk_data {
                        info.set_block_by_local_pos(block_local_pos, block_info);
                    }
                    chunks_pos.push(chunk_pos);
                } else {
                    println!("MODIFY_BLOCK_BATCH Cant find ChunkInfo in {:?}", chunk_pos);
                }
            }

            for chunk_pos in chunks_pos {
                ChunksManager::update_chunk_mesh(
                    chunks_info.clone(),
                    chunk_pos.clone(),
                    update_mesh_tx.clone(),
                    texture_mapper.clone(),
                );
                //println!("update chunk mesh:{:?}", c);
            }
        //});
    }

    fn update_chunk_mesh(
        chunks_info: ChunksInfoType,
        chunk_pos: [i32; 3],
        update_mesh_tx: Sender<([i32; 3], Geometry)>,
        texture_mapper: Arc<RwLock<TextureMapper>>,
    ) {
        let ci = match chunks_info.read() {
            Ok(e) => e,
            Err(e) => {
                println!("UPDATE_CHUNK_MESH lock error: {:?}", e);
                return;
            }
        };
        let info = match ci.get(&chunk_pos) {
            Some(e) => e,
            _ => {
                println!("UPDATE_CHUNK_MESH error: no ChunkInfo in {:?}", chunk_pos);
                return;
            }
        };
        let chunk_data = info.get_chunk_data();
        let bordered_chunk_data = format_chunk_data_with_boundaries(&ci, &chunk_data, &chunk_pos);

        let new_geometry = generate_chunk_geometry(texture_mapper, &bordered_chunk_data);
        match update_mesh_tx.send((chunk_pos, new_geometry)) {
            Ok(()) => (),
            Err(e) => {
                println!("UPDATE_CHUNK_MESH send error: {:?}", e);
                return;
            }
        }
    }

    pub fn is_loaded(&mut self, chunk_pos: &[i32; 3]) -> bool {
        match self.chunks_godot_ids.get(chunk_pos) {
            Some(index) => match self.get_chunk_by_index(*index) {
                Some(c) => c.bind().is_loaded(),
                _ => false,
            },
            _ => false,
        }
    }

    pub fn get_chunk_by_index(&self, index: i64) -> Option<Gd<Chunk>> {
        if let Some(n) = self.base.get_child(index, true) {
            return Some(n.cast::<Chunk>());
        }
        return None;
    }

    pub fn load_chunk_data<'a>(
        world_generator: Arc<RwLock<WorldGenerator>>,
        ci_write: &mut ChunksInfoLockWrite,
        chunk_pos: &[i32; 3],
    ) -> bool {
        let mut chunk_data = [BlockInfo::new(BlockType::Air); ChunkShape::SIZE as usize];
        let has_any_block = world_generator
            .read()
            .unwrap()
            .generate_chunk_data(&mut chunk_data, chunk_pos);
        ci_write.insert(*chunk_pos, ChunkInfo::new(chunk_data));
        has_any_block
    }

    pub fn spawn_chunk(&mut self, chunk_pos: &[i32; 3]) -> Gd<Chunk> {
        let mut chunk = Gd::<Chunk>::with_base(|base| Chunk::create(base));

        let chunk_name = GodotString::from(format!("chunk_{}_{}_{}", chunk_pos[0], chunk_pos[1], chunk_pos[2]));
        chunk.bind_mut().base.set_name(chunk_name.clone());

        let global_pos = ChunkInfo::get_chunk_pos_from_coordinate(&chunk_pos);

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
            chunks_info: Arc::new(RwLock::new(HashMap::new())),
            chunks_godot_ids: HashMap::new(),
            world_generator: Arc::new(RwLock::new(WorldGenerator::new(seed))),
            material: texture.duplicate(true).unwrap().cast::<Material>(),
            texture_mapper: Arc::new(RwLock::new(texture_mapper)),

            update_mesh_tx: update_mesh_tx,
            update_mesh_rx: update_mesh_rx,
        }
    }

    #[allow(unused_variables)]
    fn process(&mut self, delta: f64) {
        for (chunk_pos, new_geometry) in self.update_mesh_rx.try_iter() {
            if let Some(index) = self.chunks_godot_ids.get(&chunk_pos) {
                if let Some(mut chunk) = self.get_chunk_by_index(*index) {
                    // println!("Mesh updated: {:?}; surfaces: {}", chunk_pos, new_geometry.mesh_ist.get_surface_count());
                    chunk.bind_mut().update_mesh(new_geometry.mesh_ist);
                } else {
                    println!("Cant update mesh for chunk: index {:?} not found", index);
                }
            } else {
                println!("Cant find godot index for chunk {:?}", chunk_pos);
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
