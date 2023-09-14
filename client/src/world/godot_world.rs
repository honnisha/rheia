use common::{
    blocks::block_info::BlockInfo,
    chunks::{block_position::BlockPosition, chunk_position::ChunkPosition, utils::SectionsData},
};
use godot::{
    engine::{Material, StandardMaterial3D},
    prelude::*,
};
use parking_lot::RwLock;
use rapier3d::prelude::*;
use std::rc::Rc;
use std::{cell::RefCell, sync::Arc};

use crate::{
    controller::{player_controller::PlayerController, player_movement::PlayerMovement},
    network::client::NetworkContainer,
    utils::textures::texture_mapper::TextureMapper,
};

use super::{
    chunks::{chunk::Chunk, godot_chunks_container::ChunksContainer},
    physics_handler::PhysicsContainer,
    world_manager::TextureMapperType,
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
    camera: Gd<Camera3D>,

    physics_container: PhysicsContainer,
    player_controller: Gd<PlayerController>,
}

impl World {
    pub fn _modify_block(&mut self, pos: &BlockPosition, block_info: BlockInfo) {
        self.chunks_container.bind_mut().modify_block(pos, block_info);
    }
}

impl World {
    pub fn create(
        base: Base<Node>,
        slug: String,
        texture_mapper: TextureMapperType,
        material: Gd<Material>,
        camera: &Gd<Camera3D>,
    ) -> Self {
        let mut chunks_container = Gd::<ChunksContainer>::with_base(|base| {
            ChunksContainer::create(base, texture_mapper.clone(), material.share())
        });
        let container_name = GodotString::from("ChunksContainer");
        chunks_container.bind_mut().base.set_name(container_name.clone());
        let mut physics_container = PhysicsContainer::default();
        let player_controller =
            Gd::<PlayerController>::with_base(|base| PlayerController::create(base, &camera, &mut physics_container));

        World {
            base,
            slug: slug,
            chunks_container,
            camera: camera.share(),

            physics_container,
            player_controller,
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

    pub fn get_physics_container(&mut self) -> &PhysicsContainer {
        &self.physics_container
    }

    pub fn get_player_controller(&self) -> &Gd<PlayerController> {
        &self.player_controller
    }

    pub fn get_player_controller_mut(&mut self) -> &mut Gd<PlayerController> {
        &mut self.player_controller
    }
}

pub fn get_default_material() -> Gd<Material> {
    StandardMaterial3D::new().duplicate().unwrap().cast::<Material>()
}

#[godot_api]
impl World {
    #[func]
    fn handler_player_move(&self, movement_var: Variant) {
        let movement = movement_var.to::<PlayerMovement>();
        NetworkContainer::send_player_move(movement);
    }
}

#[godot_api]
impl NodeVirtual for World {
    /// For default godot init; only World::create is using
    fn init(base: Base<Node>) -> Self {
        let camera = load::<PackedScene>("res://scenes/camera_3d.tscn").instantiate_as::<Camera3D>();
        World::create(
            base,
            "Godot".to_string(),
            Arc::new(RwLock::new(TextureMapper::new())),
            get_default_material(),
            &camera,
        )
    }

    fn ready(&mut self) {
        self.base.add_child(self.chunks_container.share().upcast());
        self.player_controller.bind_mut().base.connect(
            "on_player_move".into(),
            Callable::from_object_method(self.base.share(), "handler_player_move"),
        );
        self.base.add_child(self.player_controller.share().upcast());
    }

    fn process(&mut self, _delta: f64) {
        let now = std::time::Instant::now();

        self.physics_container.step();

        let elapsed = now.elapsed();
        if elapsed > std::time::Duration::from_millis(20) {
            println!("World \"{}\" process: {:.2?}", self.slug, elapsed);
        }
    }
}
