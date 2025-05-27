use super::{
    block_storage::BlockStorage,
    chunks::chunks_map::ChunkMap,
    physics::PhysicsProxy,
    worlds_manager::{BlockStorageType, TextureMapperType},
};
use crate::{
    client_scripts::resource_manager::{ResourceManager, ResourceStorage},
    entities::entities_manager::EntitiesManager,
};
use common::chunks::{block_position::BlockPosition, chunk_data::{BlockDataInfo, ChunkData}, chunk_position::ChunkPosition};
use godot::{classes::Material, prelude::*};

/// Godot world
/// Contains all things inside world
///
/// ChunkMap
/// ║
/// ╚ChunkColumn
///  ║
///  ╚ChunkSection
#[derive(GodotClass)]
#[class(no_init, tool, base=Node)]
pub struct WorldManager {
    base: Base<Node>,
    slug: String,
    chunk_map: Gd<ChunkMap>,

    physics: PhysicsProxy,

    entities_manager: Gd<EntitiesManager>,

    texture_mapper: TextureMapperType,
    material: Gd<Material>,
    block_storage: BlockStorageType,
}

impl WorldManager {
    pub fn create(
        base: Base<Node>,
        slug: String,
        texture_mapper: TextureMapperType,
        material: Gd<Material>,
        block_storage: BlockStorageType,
    ) -> Self {
        let physics = PhysicsProxy::default();
        let mut chunk_map = Gd::<ChunkMap>::from_init_fn(|base| ChunkMap::create(base));
        chunk_map.bind_mut().base_mut().set_name("ChunkMap");

        Self {
            base,
            slug: slug,
            chunk_map,

            physics,

            entities_manager: Gd::<EntitiesManager>::from_init_fn(|base| EntitiesManager::create(base)),

            texture_mapper,
            material,
            block_storage,
        }
    }

    pub fn _get_entities_manager(&self) -> GdRef<EntitiesManager> {
        self.entities_manager.bind()
    }

    pub fn get_entities_manager_mut(&mut self) -> GdMut<EntitiesManager> {
        self.entities_manager.bind_mut()
    }

    pub fn get_physics(&self) -> &PhysicsProxy {
        &self.physics
    }

    pub fn get_slug(&self) -> &String {
        &self.slug
    }

    pub fn get_chunks_count(&self) -> usize {
        self.chunk_map.bind().get_chunks_count()
    }

    pub fn get_chunk_map(&self) -> GdRef<ChunkMap> {
        self.chunk_map.bind()
    }

    /// Recieve chunk data from network
    pub fn recieve_chunk(&mut self, chunk_position: ChunkPosition, data: ChunkData) {
        self.chunk_map.bind_mut().create_chunk_column(chunk_position, data);
    }

    /// Recieve chunk unloaded from network
    pub fn unload_chunk(&mut self, chunk_position: ChunkPosition) {
        self.chunk_map.bind_mut().unload_chunk(chunk_position)
    }

    pub fn edit_block(
        &self,
        position: BlockPosition,
        block_storage: &BlockStorage,
        new_block_info: Option<BlockDataInfo>,
        resource_storage: &ResourceStorage,
    ) -> Result<(), String> {
        self.chunk_map
            .bind()
            .edit_block(position, block_storage, new_block_info, &self.physics, resource_storage)
    }

    pub fn physics_process(&mut self, delta: f64) {
        #[cfg(feature = "trace")]
        let _span = tracy_client::span!("physics_step");

        let now = std::time::Instant::now();

        self.physics.step(delta as f32);

        let elapsed = now.elapsed();
        if elapsed >= std::time::Duration::from_secs_f32(0.1) {
            log::info!("physics_process lag: {:.2?}", elapsed);
        }
    }

    pub fn custom_process(&mut self, _delta: f64, resource_manager: &ResourceManager) {
        #[cfg(feature = "trace")]
        let _span = tracy_client::span!("world_manager");

        let mut map = self.chunk_map.bind_mut();
        map.send_chunks_to_load(
            self.material.instance_id(),
            self.texture_mapper.clone(),
            self.block_storage.clone(),
            &self.physics,
            resource_manager,
        );
        map.spawn_loaded_chunks(&self.physics);

        let bs = self.block_storage.read();
        let tm = self.texture_mapper.read();
        map.update_chunks_geometry(&self.physics, &bs, &tm);
    }
}

#[godot_api]
impl WorldManager {}

#[godot_api]
impl INode for WorldManager {
    fn ready(&mut self) {
        let chunk_map = self.chunk_map.clone();
        self.base_mut().add_child(&chunk_map);

        let entities_manager = self.entities_manager.clone();
        self.base_mut().add_child(&entities_manager);
    }
}
