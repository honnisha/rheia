use super::{
    chunks::chunks_map::ChunkMap,
    physics::PhysicsProxy,
    worlds_manager::{BlockStorageType, TextureMapperType},
};
use crate::{
    controller::{entity_movement::EntityMovement, player_controller::PlayerController},
    entities::entities_manager::EntitiesManager,
    network::client::NetworkLockType,
};
use common::{
    blocks::block_info::BlockInfo,
    chunks::{block_position::BlockPosition, chunk_position::ChunkPosition},
};
use godot::{classes::Material, prelude::*};
use network::{
    client::IClientNetwork,
    messages::{NetworkMessageType, SectionsData},
};

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
    pub(crate) base: Base<Node>,
    slug: String,
    chunk_map: Gd<ChunkMap>,

    physics: PhysicsProxy,
    player_controller: Gd<PlayerController>,

    entities_manager: Gd<EntitiesManager>,

    texture_mapper: TextureMapperType,
    material: Gd<Material>,
    block_storage: BlockStorageType,
    network_lock: NetworkLockType,
}

impl WorldManager {
    pub fn create(
        base: Base<Node>,
        slug: String,
        texture_mapper: TextureMapperType,
        material: Gd<Material>,
        block_storage: BlockStorageType,
        network_lock: NetworkLockType,
    ) -> Self {
        let physics = PhysicsProxy::default();
        let mut chunk_map = Gd::<ChunkMap>::from_init_fn(|base| ChunkMap::create(base));
        chunk_map.bind_mut().base_mut().set_name("ChunkMap");
        let player_controller = Gd::<PlayerController>::from_init_fn(|base| {
            PlayerController::create(base, physics.clone(), network_lock.clone())
        });

        Self {
            base,
            slug: slug,
            chunk_map,

            physics,
            player_controller,

            entities_manager: Gd::<EntitiesManager>::from_init_fn(|base| EntitiesManager::create(base)),

            texture_mapper,
            material,
            block_storage,
            network_lock,
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

    pub fn get_player_controller(&self) -> &Gd<PlayerController> {
        &self.player_controller
    }

    pub fn get_player_controller_mut(&mut self) -> &mut Gd<PlayerController> {
        &mut self.player_controller
    }
}

#[godot_api]
impl WorldManager {
    #[func]
    fn handler_player_move(&mut self, movement: Gd<EntityMovement>, _new_chunk: bool) {
        self.network_lock
            .read()
            .send_message(NetworkMessageType::Unreliable, &movement.bind().into_network());
    }

    #[func]
    fn on_chunk_loaded(&mut self) {
        unimplemented!()
    }
}

#[godot_api]
impl INode for WorldManager {
    fn ready(&mut self) {
        let obj = self.base().to_godot().clone();

        let chunk_map = self.chunk_map.clone();
        self.base_mut().add_child(&chunk_map);

        // Bind world player move signal
        self.player_controller.bind_mut().base_mut().connect(
            "on_player_move",
            &Callable::from_object_method(&obj, "handler_player_move"),
        );

        let player_controller = self.player_controller.clone();
        self.base_mut().add_child(&player_controller);

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
