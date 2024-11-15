use common::chunks::rotation::Rotation;
use godot::prelude::*;
use godot::{classes::Material, prelude::Gd};
use parking_lot::lock_api::{RwLockReadGuard, RwLockWriteGuard};
use parking_lot::RwLock;
use std::sync::Arc;

use crate::client_scripts::resource_manager::ResourceManager;
use crate::network::client::NetworkLockType;
use crate::utils::textures::texture_mapper::TextureMapper;

use super::block_storage::BlockStorage;
use super::world_manager::WorldManager;

pub type TextureMapperType = Arc<RwLock<TextureMapper>>;
pub type TextureMapperRef<'a> = RwLockReadGuard<'a, parking_lot::RawRwLock, TextureMapper>;

pub type BlockStorageType = Arc<RwLock<BlockStorage>>;
pub type BlockStorageRef<'a> = RwLockReadGuard<'a, parking_lot::RawRwLock, BlockStorage>;
pub type BlockStorageRefMut<'a> = RwLockWriteGuard<'a, parking_lot::RawRwLock, BlockStorage>;

pub struct WorldsManager {
    base: Gd<Node>,
    world: Option<Gd<WorldManager>>,

    texture_mapper: TextureMapperType,
    material: Option<Gd<Material>>,

    block_storage: BlockStorageType,
}

impl WorldsManager {
    pub fn create(base: Gd<Node>) -> Self {
        Self {
            base,
            world: None,

            material: None,
            texture_mapper: Arc::new(RwLock::new(Default::default())),

            block_storage: Arc::new(RwLock::new(Default::default())),
        }
    }

    pub fn build_textures(&mut self, resource_manager: &ResourceManager) -> Result<(), String> {
        let mut texture_mapper = self.texture_mapper.write();
        let block_storage = self.block_storage.read();

        let texture = match texture_mapper.build(&*block_storage, resource_manager) {
            Ok(i) => i,
            Err(e) => return Err(e),
        };
        self.material = Some(texture);
        return Ok(());
    }

    pub fn _get_block_storage(&self) -> BlockStorageRef {
        self.block_storage.read()
    }

    pub fn get_block_storage_mut(&self) -> BlockStorageRefMut {
        self.block_storage.write()
    }

    pub fn _get_texture_mapper(&self) -> TextureMapperRef {
        self.texture_mapper.read()
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
    pub fn teleport_player(
        &mut self,
        world_slug: String,
        position: Vector3,
        rotation: Rotation,
        network_lock: NetworkLockType,
    ) {
        if self.world.is_some() {
            if self.world.as_ref().unwrap().bind().get_slug() != &world_slug {
                // Player moving to another world; old one must be destroyed
                self.destroy_world();
                self.create_world(world_slug, network_lock);
            }
        } else {
            self.create_world(world_slug, network_lock);
        }

        self.teleport_player_controller(position, rotation)
    }

    pub fn create_world(&mut self, world_slug: String, network_lock: NetworkLockType) {
        let mut world = Gd::<WorldManager>::from_init_fn(|base| {
            WorldManager::create(
                base,
                world_slug,
                self.texture_mapper.clone(),
                self.material.as_ref().unwrap().clone(),
                self.block_storage.clone(),
                network_lock,
            )
        });

        world.bind_mut().base_mut().set_name("World");

        self.base.add_child(&world.clone());
        self.world = Some(world);

        log::info!(target: "world", "World \"{}\" created;", self.world.as_ref().unwrap().bind().get_slug());
    }

    pub fn destroy_world(&mut self) {
        let slug = self.world.as_ref().unwrap().bind().get_slug().clone();
        self.base.remove_child(&self.world.as_mut().unwrap().clone());
        self.world = None;
        log::info!(target: "world", "World \"{}\" destroyed;", slug);
    }
}
