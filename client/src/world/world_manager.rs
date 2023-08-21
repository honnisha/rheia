use common::chunks::chunk_position::ChunkPosition;
use common::network::NetworkSectionType;
use godot::engine::{Engine, StandardMaterial3D};
use godot::prelude::*;
use godot::{
    engine::Material,
    prelude::{Gd, GodotString},
};
use log::{error, info};
use parking_lot::RwLock;
use std::sync::Arc;

use crate::controller::player_controller::{PlayerController, PlayerMovement};
use crate::main_scene::FloatType;
use crate::network::client::NetworkContainer;
use crate::utils::textures::{material_builder::build_blocks_material, texture_mapper::TextureMapper};

use super::godot_world::World;

pub type TextureMapperType = Arc<RwLock<TextureMapper>>;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct WorldManager {
    #[base]
    base: Base<Node>,

    world: Option<Gd<World>>,

    texture_mapper: TextureMapperType,
    material: Gd<Material>,

    player_controller: Option<Gd<PlayerController>>,
}

impl WorldManager {
    pub fn create(base: Base<Node>) -> Self {
        let mut texture_mapper = TextureMapper::new();
        let texture = build_blocks_material(&mut texture_mapper);

        Self {
            base,
            world: None,
            material: texture.duplicate().unwrap().cast::<Material>(),
            texture_mapper: Arc::new(RwLock::new(texture_mapper)),
            player_controller: None,
        }
    }

    pub fn get_world(&self) -> Option<&Gd<World>> {
        match self.world.as_ref() {
            Some(w) => Some(&w),
            None => None,
        }
    }

    fn teleport_player_controller(&mut self, new_position: Vector3) {
        self.player_controller
            .as_mut()
            .unwrap()
            .bind_mut()
            .teleport(new_position);
    }

    /// Player can teleport in new world, between worlds or in exsting world
    /// so worlds can be created and destroyed
    pub fn teleport_player(&mut self, world_slug: String, location: [FloatType; 3]) {
        if self.world.is_some() {
            if self.world.as_ref().unwrap().bind().get_slug() != &world_slug {
                // Player moving to another world; old one must be destroyed
                self.destroy_world();
                self.create_world(world_slug);
            }
        } else {
            self.create_world(world_slug);
        }

        self.teleport_player_controller(Vector3::new(location[0], location[1], location[2]))
    }

    pub fn create_world(&mut self, world_slug: String) {
        let mut world = Gd::<World>::with_base(|base| {
            World::create(base, world_slug, self.texture_mapper.clone(), self.material.share())
        });

        let world_name = GodotString::from("World");
        world.bind_mut().set_name(world_name.clone());

        self.base.add_child(world.upcast());
        self.world = Some(self.base.get_node_as::<World>(world_name));

        info!("World \"{}\" created;", self.world.as_ref().unwrap().bind().get_slug());
    }

    pub fn destroy_world(&mut self) {
        let slug = self.world.as_ref().unwrap().bind().get_slug().clone();
        self.base.remove_child(self.world.as_mut().unwrap().share().upcast());
        self.world = None;
        info!("World \"{}\" destroyed;", slug);
    }

    /// Load chunk column by the network
    pub fn load_chunk(&mut self, chunk_position: ChunkPosition, sections: NetworkSectionType) {
        match self.world.as_mut() {
            Some(w) => w.bind_mut().load_chunk(chunk_position, sections),
            None => {
                error!("load_chunk tried to run without a world");
            }
        }
    }

    pub fn create_player_controller(&mut self) -> Gd<PlayerController> {
        let mut entity =
            load::<PackedScene>("res://scenes/player_controller.tscn").instantiate_as::<PlayerController>();

        let name = GodotString::from("PlayerController");
        entity.bind_mut().set_name(name.clone());

        self.base.add_child(entity.upcast());
        self.base.get_node_as::<PlayerController>(name)
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
        let mut player_controller = self.create_player_controller();
        player_controller.bind_mut().connect(
            "on_player_move".into(),
            Callable::from_object_method(self.base.share(), "handler_player_move"),
        );
        self.player_controller = Some(player_controller);
    }

    fn process(&mut self, _delta: f64) {
        if Engine::singleton().is_editor_hint() {
            return;
        }

        if let Some(c) = self.player_controller.as_mut() {
            c.share().bind_mut().update_debug(&self);
        }
    }
}
