use common::chunks::chunk_position::ChunkPosition;
use common::chunks::utils::SectionsData;
use godot::engine::StandardMaterial3D;
use godot::prelude::*;
use godot::{
    engine::Material,
    prelude::{Gd, GodotString},
};
use log::{error, info};
use parking_lot::RwLock;
use std::sync::Arc;

use crate::controller::player_controller::PlayerController;
use crate::controller::player_movement::PlayerMovement;
use crate::main_scene::FloatType;
use crate::network::client::NetworkContainer;
use crate::utils::textures::{material_builder::build_blocks_material, texture_mapper::TextureMapper};

use super::godot_world::World;

pub type TextureMapperType = Arc<RwLock<TextureMapper>>;

pub(crate) type ChunksSectionsChannelType = (String, ChunkPosition, SectionsData);

#[derive(GodotClass)]
#[class(base=Node)]
pub struct WorldManager {
    #[base]
    pub base: Base<Node>,
    camera: Gd<Camera3D>,

    world: Option<Gd<World>>,

    texture_mapper: TextureMapperType,
    material: Gd<Material>,

    player_controller: Gd<PlayerController>,
}

impl WorldManager {
    pub fn create(base: Base<Node>, camera: &Gd<Camera3D>) -> Self {
        let mut texture_mapper = TextureMapper::new();
        let texture = build_blocks_material(&mut texture_mapper);
        Self {
            base,
            camera: camera.share(),

            world: None,
            material: texture.duplicate().unwrap().cast::<Material>(),
            texture_mapper: Arc::new(RwLock::new(texture_mapper)),
            player_controller: Gd::<PlayerController>::with_base(|base| PlayerController::create(base, &camera)),
        }
    }

    pub fn get_world(&self) -> Option<&Gd<World>> {
        match self.world.as_ref() {
            Some(w) => Some(&w),
            None => None,
        }
    }

    fn teleport_player_controller(&mut self, position: Vector3, yaw: FloatType, pitch: FloatType) {
        self.player_controller.bind_mut().teleport(position, yaw, pitch);
    }

    /// Player can teleport in new world, between worlds or in exsting world
    /// so worlds can be created and destroyed
    pub fn teleport_player(&mut self, world_slug: String, position: Vector3, yaw: FloatType, pitch: FloatType) {
        if self.world.is_some() {
            if self.world.as_ref().unwrap().bind().get_slug() != &world_slug {
                // Player moving to another world; old one must be destroyed
                self.destroy_world();
                self.create_world(world_slug);
            }
        } else {
            self.create_world(world_slug);
        }

        self.teleport_player_controller(position, yaw, pitch)
    }

    pub fn create_world(&mut self, world_slug: String) {
        let mut world = Gd::<World>::with_base(|base| {
            World::create(base, world_slug, self.texture_mapper.clone(), self.material.share())
        });

        let world_name = GodotString::from("World");
        world.bind_mut().base.set_name(world_name.clone());

        self.base.add_child(world.share().upcast());
        self.world = Some(world);

        info!("World \"{}\" created;", self.world.as_ref().unwrap().bind().get_slug());
    }

    pub fn destroy_world(&mut self) {
        let slug = self.world.as_ref().unwrap().bind().get_slug().clone();
        self.base.remove_child(self.world.as_mut().unwrap().share().upcast());
        self.world = None;
        info!("World \"{}\" destroyed;", slug);
    }

    /// Load chunk column by the network
    pub fn load_chunk(&mut self, world_slug: String, chunk_position: ChunkPosition, sections: SectionsData) {
        let world = match self.world.as_mut() {
            Some(w) => w,
            None => {
                error!("load_chunk tried to run without a world");
                return;
            }
        };

        let mut world = world.bind_mut();
        if world_slug != *world.get_slug() {
            error!(
                "Tried to load chunk {} for non existed world {}",
                chunk_position, world_slug
            );
            return;
        }
        world.load_chunk(chunk_position, sections);
    }

    pub fn unload_chunk(&mut self, world_slug: String, chunks_positions: Vec<ChunkPosition>) {
        let world = match self.world.as_mut() {
            Some(w) => w,
            None => {
                error!("unload_chunk tried to run without a world");
                return;
            }
        };

        let mut world = world.bind_mut();
        if world_slug != *world.get_slug() {
            error!("Tried to unload chunks for non existed world {}", world_slug);
            return;
        }
        world.unload_chunk(chunks_positions);
    }

    pub fn get_player_controller(&self) -> &Gd<PlayerController> {
        &self.player_controller
    }
}

pub fn get_default_material() -> Gd<Material> {
    StandardMaterial3D::new().duplicate().unwrap().cast::<Material>()
}

#[godot_api]
impl WorldManager {
    #[func]
    fn handler_player_move(&self, movement_var: Variant) {
        let movement = movement_var.to::<PlayerMovement>();
        NetworkContainer::send_player_move(movement);
    }
}

#[godot_api]
impl NodeVirtual for WorldManager {
    fn ready(&mut self) {
        self.player_controller.bind_mut().base.connect(
            "on_player_move".into(),
            Callable::from_object_method(self.base.share(), "handler_player_move"),
        );
        self.base.add_child(self.player_controller.share().upcast());
    }
}
