use common::chunks::block_position::{BlockPosition, BlockPositionTrait};
use godot::classes::StandardMaterial3D;
use godot::prelude::*;
use godot::{classes::Material, prelude::Gd};
use parking_lot::RwLock;
use parking_lot::lock_api::{RwLockReadGuard, RwLockWriteGuard};
use std::sync::Arc;

use crate::client_scripts::resource_manager::{ResourceManager, ResourceStorage};
use crate::controller::player_controller::PlayerController;
use crate::scenes::components::block_mesh_storage::BlockMeshStorage;
use crate::scenes::main_scene::ResourceManagerType;
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

    resource_manager: Option<ResourceManagerType>,

    #[init(val = Arc::new(RwLock::new(Default::default())))]
    texture_mapper: TextureMapperType,

    #[init(val = Arc::new(RwLock::new(Default::default())))]
    block_storage: BlockStorageType,

    #[export]
    terrain_material: Option<Gd<StandardMaterial3D>>,

    block_mesh_storage: Option<Gd<BlockMeshStorage>>,
}

impl WorldsManager {
    pub fn build_textures(&mut self, resources_storage: &ResourceStorage) -> Result<(), String> {
        let now = std::time::Instant::now();

        let mut texture_mapper = self.texture_mapper.write();
        let block_storage = self.block_storage.read();

        texture_mapper.clear();

        let mut material_3d = self
            .terrain_material
            .as_mut()
            .expect("Terrain StandardMaterial3D is not set");
        match texture_mapper.build(&*block_storage, resources_storage, &mut material_3d) {
            Ok(i) => i,
            Err(e) => return Err(e),
        };
        log::info!(target: "main", "Textures builded successfily; texture blocks:{} textures loaded:{} (executed:{:.2?})", block_storage.textures_blocks_count(), texture_mapper.len(), now.elapsed());
        return Ok(());
    }

    pub fn on_network_connected(&mut self, resource_manager: &ResourceManager) {
        let block_mesh_storage = {
            BlockMeshStorage::init(
                &*self.get_block_storage(),
                &self.get_material(),
                &*resource_manager,
                &*self.get_texture_mapper(),
            )
        };
        self.block_mesh_storage = Some(block_mesh_storage);
    }

    pub fn get_block_mesh_storage(&self) -> Option<&Gd<BlockMeshStorage>> {
        self.block_mesh_storage.as_ref()
    }

    pub fn get_block_storage_lock(&self) -> &BlockStorageType {
        &self.block_storage
    }

    pub fn get_block_storage(&self) -> RwLockReadGuard<'_, parking_lot::RawRwLock, BlockStorage> {
        self.block_storage.read()
    }

    pub fn set_resource_manager(&mut self, resource_manager: ResourceManagerType) {
        self.resource_manager = Some(resource_manager);
    }

    pub fn get_block_storage_mut(&self) -> RwLockWriteGuard<'_, parking_lot::RawRwLock, BlockStorage> {
        self.block_storage.write()
    }

    pub fn get_texture_mapper(&self) -> RwLockReadGuard<'_, parking_lot::RawRwLock, TextureMapper> {
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

    pub fn get_player_controller_mut(&mut self) -> &mut Option<Gd<PlayerController>> {
        &mut self.player_controller
    }

    pub fn create_player(&mut self, world: &Gd<WorldManager>) -> Gd<PlayerController> {
        let player_controller = Gd::<PlayerController>::from_init_fn(|base| {
            PlayerController::create(base, world.bind().get_physics().clone())
        });

        self.base_mut().add_child(&player_controller.clone());

        self.player_controller = Some(player_controller.clone());
        player_controller
    }

    pub fn get_material(&self) -> Gd<Material> {
        let material_3d = self
            .terrain_material
            .as_ref()
            .expect("Terrain StandardMaterial3D is not set")
            .clone();
        material_3d.upcast::<Material>()
    }

    pub fn create_world(&mut self, world_slug: String) -> Gd<WorldManager> {
        let now = std::time::Instant::now();

        let mut world = Gd::<WorldManager>::from_init_fn(|base| {
            WorldManager::create(
                base,
                world_slug.clone(),
                self.texture_mapper.clone(),
                self.get_material(),
                self.block_storage.clone(),
            )
        });

        world
            .bind_mut()
            .base_mut()
            .set_name(&format!("World \"{}\"", world_slug));

        self.base_mut().add_child(&world.clone());
        self.world = Some(world.clone());

        log::info!(target: "world", "World \"{}\" created; (executed:{:.2?})", self.world.as_ref().unwrap().bind().get_slug(), now.elapsed());

        world
    }

    pub fn destroy_world(&mut self) {
        let now = std::time::Instant::now();

        let mut base = self.base_mut().clone();

        let world_slug;
        if let Some(world) = self.world.as_mut().take() {
            world_slug = world.bind().get_slug().clone();
            base.remove_child(&world.clone());
        } else {
            panic!("destroy_world: world is not exists");
        }

        if let Some(player_controller) = self.player_controller.as_mut().take() {
            base.remove_child(&player_controller.clone());
        }
        log::info!(target: "world", "World \"{}\" destroyed; (executed:{:.2?})", world_slug, now.elapsed());
    }
}

#[godot_api]
impl INode for WorldsManager {
    fn ready(&mut self) {}

    fn physics_process(&mut self, delta: f64) {
        #[cfg(feature = "trace")]
        let _span = tracy_client::span!("worlds_manager");

        let now = std::time::Instant::now();

        if self.get_world().is_some() {
            let mut world = self.get_world_mut().unwrap().clone();
            world.bind_mut().physics_process(delta);
        }

        let elapsed = now.elapsed();
        #[cfg(debug_assertions)]
        if elapsed >= crate::WARNING_TIME {
            log::warn!(target: "worlds_manager", "&7physics_process lag: {:.2?}", elapsed);
        }
    }

    fn process(&mut self, delta: f64) {
        #[cfg(feature = "trace")]
        let _span = tracy_client::span!("worlds_manager");

        if self.get_world().is_some() {
            let mut world = self.get_world_mut().unwrap().clone();

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

            if let Some(resource_manager) = self.resource_manager.as_ref() {
                world.bind_mut().custom_process(delta, &*resource_manager.borrow());
            }
        }
    }
}
