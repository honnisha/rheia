use common::{
    blocks::block_info::BlockInfo,
    chunks::{block_position::BlockPosition, chunk_position::ChunkPosition, utils::SectionsData},
    network::messages::NetworkMessageType, physics::physics::PhysicsContainer,
};
use godot::{
    engine::{Material, StandardMaterial3D},
    prelude::*,
};
use parking_lot::RwLock;
use std::rc::Rc;
use std::{cell::RefCell, sync::Arc};

use crate::{
    controller::{player_controller::PlayerController, player_movement::PlayerMovement},
    main_scene::{Main, PhysicsContainerType},
    utils::textures::texture_mapper::TextureMapper,
};

use super::{
    chunks::{chunk::Chunk, godot_chunks_container::ChunksContainer},
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
    pub(crate) base: Base<Node>,
    slug: String,
    chunks_container: Gd<ChunksContainer>,

    physics_container: PhysicsContainerType,
    player_controller: Gd<PlayerController>,
}

impl World {
    pub fn _modify_block(&mut self, pos: &BlockPosition, block_info: BlockInfo) {
        self.chunks_container.bind_mut().modify_block(pos, block_info);
    }
}

impl World {
    pub fn create(base: Base<Node>, slug: String, texture_mapper: TextureMapperType, material: Gd<Material>) -> Self {
        let mut physics_container = PhysicsContainerType::create();
        let mut chunks_container = Gd::<ChunksContainer>::from_init_fn(|base| {
            ChunksContainer::create(
                base,
                texture_mapper.clone(),
                material.clone(),
                physics_container.clone(),
            )
        });
        let container_name = GString::from("ChunksContainer");
        chunks_container.bind_mut().base_mut().set_name(container_name.clone());
        let player_controller =
            Gd::<PlayerController>::from_init_fn(|base| PlayerController::create(base, &mut physics_container));

        World {
            base,
            slug: slug,
            chunks_container,

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

    pub fn get_player_controller(&self) -> &Gd<PlayerController> {
        &self.player_controller
    }

    pub fn get_player_controller_mut(&mut self) -> &mut Gd<PlayerController> {
        &mut self.player_controller
    }
}

pub fn get_default_material() -> Gd<Material> {
    StandardMaterial3D::new_gd().duplicate().unwrap().cast::<Material>()
}

#[godot_api]
impl World {
    #[func]
    fn handler_player_move(&mut self, movement: Gd<PlayerMovement>) {
        let main = self.base().to_godot().get_parent().unwrap().cast::<Main>();
        let main = main.bind();
        main.network_send_message(&movement.bind().into_network(), NetworkMessageType::Unreliable);
    }
}

#[godot_api]
impl INode for World {
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
        let chunks_container = self.chunks_container.clone().upcast();
        self.base_mut().add_child(chunks_container);

        // Bind world player move signal
        let obj = self.base().to_godot().clone();
        self.player_controller.bind_mut().base_mut().connect(
            "on_player_move".into(),
            Callable::from_object_method(&obj, "handler_player_move"),
        );

        let player_controller = self.player_controller.clone().upcast();
        self.base_mut().add_child(player_controller);
    }

    fn process(&mut self, delta: f64) {
        let now = std::time::Instant::now();

        self.physics_container.step(delta as f32);

        let elapsed = now.elapsed();
        if elapsed > std::time::Duration::from_millis(30) {
            log::debug!(target: "world", "World \"{}\" process: {:.2?}", self.slug, elapsed);
        }
    }
}
