use common::{
    blocks::block_info::BlockInfo,
    chunks::{block_position::BlockPosition, chunk_position::ChunkPosition, utils::SectionsData},
};
use godot::{
    engine::{Engine, Material, MeshInstance3D, SphereMesh, PhysicsServer3DManager},
    prelude::*,
};
use parking_lot::RwLock;
use rapier3d::prelude::RigidBodyHandle;
use rapier3d::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use crate::utils::textures::texture_mapper::TextureMapper;

use super::{
    chunks::godot_chunks_container::{Chunk, ChunksContainer},
    physics_handler::PhysicsController,
    world_manager::{get_default_material, TextureMapperType},
};

/// Godot world
/// Contains all things inside world
///
/// ChunksContainer
/// ║
/// ╚ChunkColumn
///  ║
///  ╚ChunkSection
#[derive(GodotClass)]
#[class(base=Node)]
pub struct World {
    #[base]
    pub(crate) base: Base<Node>,
    slug: String,
    chunks_container: Gd<ChunksContainer>,

    physics: PhysicsController,
    handle: Option<RigidBodyHandle>,
    obj: Option<Gd<MeshInstance3D>>,
}

impl World {
    pub fn _modify_block(&mut self, pos: &BlockPosition, block_info: BlockInfo) {
        self.chunks_container.bind_mut().modify_block(pos, block_info);
    }
}

impl World {
    pub fn create(base: Base<Node>, slug: String, texture_mapper: TextureMapperType, material: Gd<Material>) -> Self {
        let mut chunks_container = Gd::<ChunksContainer>::with_base(|base| {
            ChunksContainer::create(base, texture_mapper.clone(), material.share())
        });
        let container_name = GodotString::from("ChunksContainer");
        chunks_container.bind_mut().base.set_name(container_name.clone());
        World {
            base,
            slug: slug,
            chunks_container,
            physics: PhysicsController::default(),
            handle: None,
            obj: None,
        }
    }

    pub fn get_slug(&self) -> &String {
        &self.slug
    }

    pub fn get_chunks_count(&self) -> usize {
        self.chunks_container.bind().get_chunks_count()
    }

    pub fn get_chunk(&self, chunk_position: &ChunkPosition) -> Option<Rc<RefCell<Chunk>>> {
        if let Some(chunk) = self.chunks_container.bind().get_chunk(chunk_position) {
            return Some(chunk.clone());
        }
        return None;
    }

    pub fn load_chunk(&mut self, chunk_position: ChunkPosition, sections: SectionsData) {
        self.chunks_container.bind_mut().load_chunk(chunk_position, sections);
    }

    pub fn unload_chunk(&mut self, chunks_positions: Vec<ChunkPosition>) {
        self.chunks_container.bind_mut().unload_chunk(chunks_positions);
    }

    pub fn get_physics_mut(&mut self) -> &mut PhysicsController {
        &mut self.physics
    }
}

#[godot_api]
impl NodeVirtual for World {
    /// For default godot init; only World::create is using
    fn init(base: Base<Node>) -> Self {
        World::create(
            base,
            "Godot".to_string(),
            Arc::new(RwLock::new(TextureMapper::new())),
            get_default_material(),
        )
    }

    fn ready(&mut self) {
        self.base.add_child(self.chunks_container.share().upcast());

        self.obj = Some(MeshInstance3D::new_alloc());
        let mut sphere = self.obj.as_mut().unwrap();
        let mesh = SphereMesh::new();
        sphere.set_mesh(mesh.upcast());
        self.base.add_child(sphere.share().upcast());
        sphere.set_position(Vector3 {
            x: 0.0,
            y: 60.0,
            z: 0.0,
        });

        let (rigid_handle, _collider_handle) = self.physics.create_ball(&sphere.get_position());
        self.handle = Some(rigid_handle);
    }

    fn process(&mut self, _delta: f64) {
        self.physics.step();
        if let Some(h) = self.handle {
            let body = self.physics.get_rigid_body(&h).unwrap();
            let mut sphere = self.obj.as_mut().unwrap();
            let new_pos = Vector3::new(body.translation()[0], body.translation()[1], body.translation()[2]);
            sphere.set_position(new_pos);
        }

        let input = Input::singleton();
        if input.is_action_just_pressed("ui_up".into()) {
            if let Some(h) = self.handle {
                let body = self.physics.get_rigid_body_mut(&h).unwrap();
                body.apply_impulse(Vector::new(-0.5, 10.0, 0.0), true);
            }
        }
    }
}
