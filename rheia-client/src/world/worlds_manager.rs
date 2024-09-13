use common::chunks::rotation::Rotation;
use godot::prelude::*;
use godot::{engine::Material, prelude::Gd};
use parking_lot::RwLock;
use std::sync::Arc;

use crate::utils::textures::{material_builder::build_blocks_material, texture_mapper::TextureMapper};

use super::world_manager::WorldManager;

pub type TextureMapperType = Arc<RwLock<TextureMapper>>;

pub struct WorldsManager {
    base: Gd<Node>,
    world: Option<Gd<WorldManager>>,

    texture_mapper: TextureMapperType,
    material: Gd<Material>,
}

impl WorldsManager {
    pub fn create(base: Gd<Node>) -> Self {
        let mut texture_mapper = TextureMapper::new();
        let texture = build_blocks_material(&mut texture_mapper);
        Self {
            base,
            world: None,

            material: texture.duplicate().unwrap().cast::<Material>(),
            texture_mapper: Arc::new(RwLock::new(texture_mapper)),
        }
    }

    pub fn get_world(&self) -> Option<&Gd<WorldManager>> {
        match self.world.as_ref() {
            Some(w) => Some(&w),
            None => None,
        }
    }

    pub fn get_world_mut(&mut self) -> Option<&mut Gd<WorldManager>> {
        match self.world.as_mut() {
            Some(w) => Some(w),
            None => None,
        }
    }

    /// Raise exception if there is no world
    fn teleport_player_controller(&mut self, position: Vector3, rotation: Rotation) {
        let mut world = self.world.as_mut().unwrap().bind_mut();
        let mut player_controller = world.get_player_controller_mut().bind_mut();

        player_controller.set_position(position);
        player_controller.set_rotation(rotation);
    }

    /// Player can teleport in new world, between worlds or in exsting world
    /// so worlds can be created and destroyed
    pub fn teleport_player(&mut self, world_slug: String, position: Vector3, rotation: Rotation) {
        if self.world.is_some() {
            if self.world.as_ref().unwrap().bind().get_slug() != &world_slug {
                // Player moving to another world; old one must be destroyed
                self.destroy_world();
                self.create_world(world_slug);
            }
        } else {
            self.create_world(world_slug);
        }

        self.teleport_player_controller(position, rotation)
    }

    pub fn create_world(&mut self, world_slug: String) {
        let mut world = Gd::<WorldManager>::from_init_fn(|base| {
            WorldManager::create(base, world_slug, self.texture_mapper.clone(), self.material.clone())
        });

        let world_name = GString::from("World");
        world.bind_mut().base_mut().set_name(world_name.clone());

        self.base.add_child(world.clone().upcast());
        self.world = Some(world);

        log::info!(target: "world", "World \"{}\" created;", self.world.as_ref().unwrap().bind().get_slug());
    }

    pub fn destroy_world(&mut self) {
        let slug = self.world.as_ref().unwrap().bind().get_slug().clone();
        self.base.remove_child(self.world.as_mut().unwrap().clone().upcast());
        self.world = None;
        log::info!(target: "world", "World \"{}\" destroyed;", slug);
    }
}
