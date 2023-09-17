use common::chunks::chunk_position::ChunkPosition;
use common::chunks::utils::SectionsData;
use godot::prelude::*;
use godot::{
    engine::Material,
    prelude::{Gd, GodotString},
};
use log::{error, info};
use parking_lot::RwLock;
use std::sync::Arc;

use crate::main_scene::FloatType;
use crate::utils::textures::{material_builder::build_blocks_material, texture_mapper::TextureMapper};

use super::godot_world::World;

pub type TextureMapperType = Arc<RwLock<TextureMapper>>;

pub struct WorldManager {
    base: Gd<Node>,
    world: Option<Gd<World>>,

    texture_mapper: TextureMapperType,
    material: Gd<Material>,
}

impl WorldManager {
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

    pub fn get_world(&self) -> Option<&Gd<World>> {
        match self.world.as_ref() {
            Some(w) => Some(&w),
            None => None,
        }
    }

    /// Raise exception if there is no world
    fn teleport_player_controller(&mut self, position: Vector3, yaw: FloatType, pitch: FloatType) {
        self.world
            .as_mut()
            .unwrap()
            .bind_mut()
            .get_player_controller_mut()
            .bind_mut()
            .teleport(position, yaw, pitch);
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
            World::create(
                base,
                world_slug,
                self.texture_mapper.clone(),
                self.material.share(),
            )
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
}
