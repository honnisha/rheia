use common::chunks::block_position::{BlockPosition, BlockPositionTrait};
use common::chunks::rotation::Rotation;
use godot::prelude::*;
use godot::{classes::Material, prelude::Gd};
use parking_lot::lock_api::{RwLockReadGuard, RwLockWriteGuard};
use parking_lot::RwLock;
use std::sync::Arc;

use crate::client_scripts::resource_manager::ResourceManager;
use crate::controller::player_controller::PlayerController;
use crate::utils::textures::texture_mapper::TextureMapper;

use super::block_storage::BlockStorage;
use super::world_manager::WorldManager;

pub type TextureMapperType = Arc<RwLock<TextureMapper>>;
pub type BlockStorageType = Arc<RwLock<BlockStorage>>;

#[derive(GodotClass)]
#[class(init, tool, base=Node)]
pub struct WorldsManager {
    base: Base<Node>,

    world: Option<Gd<WorldManager>>,
    player_controller: Option<Gd<PlayerController>>,

    #[init(val = Arc::new(RwLock::new(Default::default())))]
    texture_mapper: TextureMapperType,

    #[init(val = Arc::new(RwLock::new(Default::default())))]
    block_storage: BlockStorageType,

    material: Option<Gd<Material>>,
}

impl WorldsManager {
    pub fn build_textures(&mut self, resource_manager: &ResourceManager) -> Result<(), String> {
        let mut texture_mapper = self.texture_mapper.write();
        let block_storage = self.block_storage.read();

        let texture = match texture_mapper.build(&*block_storage, resource_manager) {
            Ok(i) => i,
            Err(e) => return Err(e),
        };
        self.material = Some(texture);
        log::info!(target: "main", "Textures builded successfily; texture blocks:{} textures loaded:{}", block_storage.textures_blocks_count(), texture_mapper.len());
        return Ok(());
    }

    pub fn _get_block_storage(&self) -> RwLockReadGuard<'_, parking_lot::RawRwLock, BlockStorage> {
        self.block_storage.read()
    }

    pub fn get_block_storage_mut(&self) -> RwLockWriteGuard<'_, parking_lot::RawRwLock, BlockStorage> {
        self.block_storage.write()
    }

    pub fn _get_texture_mapper(&self) -> RwLockReadGuard<'_, parking_lot::RawRwLock, TextureMapper> {
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

    pub fn get_player_controller(&self) -> &Option<Gd<PlayerController>> {
        &self.player_controller
    }

    /// Raise exception if there is no world
    pub fn teleport_player_controller(&mut self, position: Vector3, rotation: Rotation) {
        let player_controller = self.player_controller.as_mut().unwrap();

        player_controller.bind_mut().set_position(position);
        player_controller.bind_mut().set_rotation(rotation);
    }

    pub fn create_player(&mut self, world: &Gd<WorldManager>) -> Gd<PlayerController> {
        let player_controller = Gd::<PlayerController>::from_init_fn(|base| {
            PlayerController::create(base, world.bind().get_physics().clone())
        });

        self.base_mut().add_child(&player_controller.clone());

        self.player_controller = Some(player_controller.clone());
        player_controller
    }

    pub fn create_world(&mut self, world_slug: String) -> Gd<WorldManager> {
        let mut world = Gd::<WorldManager>::from_init_fn(|base| {
            WorldManager::create(
                base,
                world_slug.clone(),
                self.texture_mapper.clone(),
                self.material.as_ref().expect("material must be builded").clone(),
                self.block_storage.clone(),
            )
        });

        world
            .bind_mut()
            .base_mut()
            .set_name(&format!("World \"{}\"", world_slug));

        self.base_mut().add_child(&world.clone());
        self.world = Some(world.clone());

        log::info!(target: "world", "World \"{}\" created;", self.world.as_ref().unwrap().bind().get_slug());

        world
    }

    pub fn destroy_world(&mut self) {
        let mut base = self.base_mut().clone();

        if let Some(player_controller) = self.player_controller.as_mut().take() {
            base.remove_child(&player_controller.clone());
        }

        if let Some(world) = self.world.as_mut().take() {
            base.remove_child(&world.clone());
        }
    }
}

#[godot_api]
impl INode for WorldsManager {
    fn ready(&mut self) {}

    fn process(&mut self, delta: f64) {
        if self.get_world().is_some() {
            let world = self.get_world().unwrap().clone();

            if let Some(player_controller) = self.player_controller.as_mut() {
                let mut player_controller = player_controller.bind_mut();

                let pos = player_controller.get_position();
                let chunk_pos = BlockPosition::new(pos.x as i64, pos.y as i64, pos.z as i64).get_chunk_position();
                let chunk_loaded = match world.bind().get_chunk_map().get_chunk(&chunk_pos) {
                    Some(c) => c.read().is_loaded(),
                    None => false,
                };
                player_controller.custom_process(delta, chunk_loaded, world.bind().get_slug());
            }
        }
    }
}
