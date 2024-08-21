use super::{
    chunks::chunks_map::{ChunkLock, ChunkMap},
    worlds_manager::TextureMapperType,
};
use crate::{
    controller::{entity_movement::EntityMovement, player_controller::PlayerController},
    entities::entities_manager::EntitiesManager,
    main_scene::{Main, PhysicsContainerType}, utils::bridge::IntoChunkPositionVector,
};
use common::{
    chunks::{chunk_position::ChunkPosition, utils::SectionsData},
    network::messages::{NetworkMessageType, Rotation},
    physics::physics::PhysicsContainer,
};
use godot::{engine::Material, prelude::*};

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

    physics_container: PhysicsContainerType,
    player_controller: Gd<PlayerController>,

    entities_manager: Gd<EntitiesManager>,
}

impl WorldManager {}

impl WorldManager {
    pub fn create(base: Base<Node>, slug: String, texture_mapper: TextureMapperType, material: Gd<Material>) -> Self {
        let mut physics_container = PhysicsContainerType::create();
        let mut chunk_map = Gd::<ChunkMap>::from_init_fn(|base| {
            ChunkMap::create(
                base,
                texture_mapper.clone(),
                material.clone(),
                physics_container.clone(),
            )
        });
        let container_name = GString::from("ChunkMap");
        chunk_map.bind_mut().base_mut().set_name(container_name.clone());
        let player_controller =
            Gd::<PlayerController>::from_init_fn(|base| PlayerController::create(base, &mut physics_container));

        Self {
            base,
            slug: slug,
            chunk_map,

            physics_container,
            player_controller,

            entities_manager: Gd::<EntitiesManager>::from_init_fn(|base| EntitiesManager::create(base)),
        }
    }

    pub fn get_slug(&self) -> &String {
        &self.slug
    }

    pub fn get_chunks_count(&self) -> usize {
        self.chunk_map.bind().get_chunks_count()
    }

    pub fn get_chunk(&self, chunk_position: &ChunkPosition) -> Option<ChunkLock> {
        if let Some(chunk) = self.chunk_map.bind().get_chunk(chunk_position) {
            return Some(chunk.clone());
        }
        return None;
    }

    pub fn load_chunk(&mut self, chunk_position: ChunkPosition, sections: SectionsData) {
        self.chunk_map.bind_mut().load_chunk(chunk_position, sections);
    }

    pub fn unload_chunk(&mut self, chunk_position: ChunkPosition) {
        self.chunk_map.bind_mut().unload_chunk(chunk_position);
    }

    pub fn get_player_controller(&self) -> &Gd<PlayerController> {
        &self.player_controller
    }

    pub fn get_player_controller_mut(&mut self) -> &mut Gd<PlayerController> {
        &mut self.player_controller
    }

    pub fn start_streaming_entity(&mut self, id: u32, position: Vector3, rotation: Rotation) {
        log::info!(
            "start_streaming_entity id:{} position:{} rotation:{}",
            id,
            position,
            rotation
        );
    }

    pub fn move_entity(&mut self, id: u32, position: Vector3, rotation: Rotation) {
        log::info!("move_entity id:{} position:{} rotation:{}", id, position, rotation);
    }

    pub fn stop_streaming_entities(&mut self, ids: Vec<u32>) {
        log::info!("stop_streaming_entities ids:{:?}", ids);
    }
}

#[godot_api]
impl WorldManager {
    #[func]
    fn handler_player_move(&mut self, movement: Gd<EntityMovement>, new_chunk: bool) {
        let main = self.base().to_godot().get_parent().unwrap().cast::<Main>();
        let main = main.bind();
        main.network_send_message(&movement.bind().into_network(), NetworkMessageType::Unreliable);

        if new_chunk {
            self.chunk_map
                .bind_mut()
                .change_active_chunk(&movement.bind().get_position().to_chunk_position());
        }
    }
}

#[godot_api]
impl INode for WorldManager {
    fn ready(&mut self) {
        let chunk_map = self.chunk_map.clone().upcast();
        self.base_mut().add_child(chunk_map);

        // Bind world player move signal
        let obj = self.base().to_godot().clone();
        self.player_controller.bind_mut().base_mut().connect(
            "on_player_move".into(),
            Callable::from_object_method(&obj, "handler_player_move"),
        );

        let player_controller = self.player_controller.clone().upcast();
        self.base_mut().add_child(player_controller);

        let entities_manager = self.entities_manager.clone().upcast();
        self.base_mut().add_child(entities_manager);
    }

    fn process(&mut self, delta: f64) {
        let now = std::time::Instant::now();

        self.physics_container.step(delta as f32);

        let elapsed = now.elapsed();
        if elapsed > std::time::Duration::from_millis(30) {
            log::debug!(target: "world", "World \"{}\" process: {:.2?}", self.slug, elapsed);
        }
    }
}
