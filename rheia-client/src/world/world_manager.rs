use super::{
    chunks::chunks_map::ChunkMap,
    physics::PhysicsProxy,
    worlds_manager::{BlockStorageType, TextureMapperType},
};
use crate::{controller::entity_movement::EntityMovement, entities::entities_manager::EntitiesManager};
use common::{
    blocks::block_info::BlockInfo,
    chunks::{block_position::BlockPosition, chunk_position::ChunkPosition},
};
use godot::{classes::Material, prelude::*};
use network::messages::{NetworkMessageType, SectionsData};

/// Godot world
/// Contains all things inside world
///
/// ChunkMap
/// ║
/// ╚ChunkColumn
///  ║
///  ╚ChunkSection
#[derive(GodotClass)]
#[class(no_init, base=Node)]
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
    pub fn recieve_chunk(&mut self, chunk_position: ChunkPosition, data: SectionsData) {
        self.chunk_map.bind_mut().create_chunk_column(chunk_position, data);
    }

    /// Recieve chunk unloaded from network
    pub fn unload_chunk(&mut self, chunk_position: ChunkPosition) {
        self.chunk_map.bind_mut().unload_chunk(chunk_position)
    }

    pub fn edit_block(&mut self, position: BlockPosition, new_block_info: BlockInfo) {
        self.chunk_map.bind_mut().edit_block(position, new_block_info)
    }
}

#[godot_api]
impl WorldManager {
}

#[godot_api]
impl INode for WorldManager {
    fn ready(&mut self) {
        let chunk_map = self.chunk_map.clone();
        self.base_mut().add_child(&chunk_map);

        let entities_manager = self.entities_manager.clone();
        self.base_mut().add_child(&entities_manager);
    }

    fn process(&mut self, delta: f64) {
        #[cfg(feature = "trace")]
        let _span = tracing::span!(tracing::Level::INFO, "world_manager").entered();

        self.physics.step(delta as f32);

        let mut map = self.chunk_map.bind_mut();
        map.send_chunks_to_load(
            self.material.instance_id(),
            self.texture_mapper.clone(),
            self.block_storage.clone(),
        );
        map.spawn_loaded_chunks(&self.physics);

        let bs = self.block_storage.read();
        let tm = self.texture_mapper.read();
        map.update_chunks(&self.physics, &bs, &tm);
    }
}
