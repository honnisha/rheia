use common::{chunks::chunk_position::ChunkPosition, network::NetworkSectionType};
use godot::engine::StandardMaterial3D;
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
    world: Option<Gd<World>>,

    texture_mapper: TextureMapperType,
    material: Gd<Material>,
}

impl WorldManager {
    pub fn new() -> Self {
        let mut texture_mapper = TextureMapper::new();
        let texture = build_blocks_material(&mut texture_mapper);

        Self {
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

    /// Player can teleport in new world, between worlds or in exsting world
    /// so worlds can be created and destroyed
    pub fn teleport_player(&mut self, main: &mut Base<Node>, world_slug: String, location: [FloatType; 3]) {
        if self.world.is_some() {
            if self.world.as_ref().unwrap().bind().get_slug() != &world_slug {
                // Player moving to another world; old one must be destroyed
                self.destroy_world(main);
                self.create_world(main, world_slug);
            }
        } else {
            self.create_world(main, world_slug);
        }

        // TODO: teleport player
    }

    pub fn create_world(&mut self, main: &mut Base<Node>, world_slug: String) {
        let mut world = Gd::<World>::with_base(|base| {
            World::create(base, world_slug, self.texture_mapper.clone(), self.material.share())
        });

        let world_name = GodotString::from("World");
        world.bind_mut().set_name(world_name.clone());

        main.add_child(world.upcast());
        self.world = Some(main.get_node_as::<World>(world_name));

        info!("World \"{}\" created;", self.world.as_ref().unwrap().bind().get_slug());
    }

    pub fn destroy_world(&mut self, main: &mut Base<Node>) {
        let slug = self.world.as_ref().unwrap().bind().get_slug().clone();
        main.remove_child(self.world.as_mut().unwrap().share().upcast());
        self.world = None;
        info!("World \"{}\" destroyed;", slug);
    }

    /// Load chunk column by the network
    pub fn load_chunk(&mut self, chunk_position: ChunkPosition, sections: NetworkSectionType) {
        match self.world.as_mut() {
            Some(w) => w.bind_mut().load_chunk(chunk_position, sections),
            None => {
                error!("load_chunk tried to run witout a world");
            }
        }
    }
}

pub fn get_default_material() -> Gd<Material> {
    StandardMaterial3D::new().duplicate().unwrap().cast::<Material>()
}
