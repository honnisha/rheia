use super::{
    chunks::chunks_map::ChunkMap,
    physics::{PhysicsProxy, PhysicsType},
    worlds_manager::TextureMapperType,
};
use crate::{
    controller::{entity_movement::EntityMovement, player_controller::PlayerController},
    entities::{entities_manager::EntitiesManager, entity::Entity},
    main_scene::Main,
    utils::{
        bridge::{IntoChunkPositionVector, IntoNetworkVector},
        primitives::generate_box_mesh,
    },
};
use common::{chunks::block_position::BlockPosition, network::messages::NetworkMessageType, physics::QueryFilter};
use godot::{
    engine::{Material, MeshInstance3D},
    prelude::*,
};
pub enum RaycastResult {
    Block(BlockPosition),
    Entity(u32, Gd<Entity>),
}

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

    block_selection: Gd<Node3D>,
}

impl WorldManager {}

impl WorldManager {
    pub fn create(base: Base<Node>, slug: String, texture_mapper: TextureMapperType, material: Gd<Material>) -> Self {
        let mut physics = PhysicsProxy::default();
        let mut chunk_map =
            Gd::<ChunkMap>::from_init_fn(|base| ChunkMap::create(base, texture_mapper.clone(), material.clone()));
        let container_name = GString::from("ChunkMap");
        chunk_map.bind_mut().base_mut().set_name(container_name.clone());
        let player_controller =
            Gd::<PlayerController>::from_init_fn(|base| PlayerController::create(base, &mut physics));

        let mut selection = Node3D::new_alloc();
        let mut mesh = generate_box_mesh();
        selection.add_child(mesh.clone().upcast());
        mesh.set_position(Vector3::new(0.5, 0.5, 0.5));

        Self {
            base,
            slug: slug,
            chunk_map,

            physics,
            player_controller,

            entities_manager: Gd::<EntitiesManager>::from_init_fn(|base| EntitiesManager::create(base)),

            block_selection: selection,
        }
    }

    pub fn get_block_selection_mut(&mut self) -> &mut Gd<Node3D> {
        &mut self.block_selection
    }

    pub fn raycast(
        &self,
        dir: Vector3,
        max_toi: f32,
        from: Vector3,
        filter: QueryFilter,
    ) -> Option<(RaycastResult, Vector3)> {
        let Some((collider_type, position)) = self.physics.raycast(dir, max_toi, from, filter) else {
            return None;
        };

        let result = match collider_type {
            PhysicsType::ChunkMeshCollider(_chunk_position) => {
                RaycastResult::Block(BlockPosition::from_global(&position.to_network()))
            }
            PhysicsType::EntityCollider(entity_id) => {
                let manager = self.entities_manager.bind();
                let Some(entity) = manager.get(entity_id) else {
                    panic!("Entity is not found for id \"{}\"", entity_id);
                };
                RaycastResult::Entity(entity_id, entity.clone())
            }
        };
        Some((result, position))
    }

    pub fn _get_entities_manager(&self) -> GdRef<EntitiesManager> {
        self.entities_manager.bind()
    }

    pub fn get_entities_manager_mut(&mut self) -> GdMut<EntitiesManager> {
        self.entities_manager.bind_mut()
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

    pub fn get_chunk_map_mut(&mut self) -> GdMut<ChunkMap> {
        self.chunk_map.bind_mut()
    }

    pub fn get_player_controller(&self) -> &Gd<PlayerController> {
        &self.player_controller
    }

    pub fn get_player_controller_mut(&mut self) -> &mut Gd<PlayerController> {
        &mut self.player_controller
    }

    pub fn get_main(&self) -> Gd<Main> {
        let main = self.base().to_godot().get_parent().unwrap().cast::<Main>();
        main.clone()
    }
}

#[godot_api]
impl WorldManager {
    #[func]
    fn handler_player_move(&mut self, movement: Gd<EntityMovement>, new_chunk: bool) {
        let main = self.get_main();
        main.bind()
            .network_send_message(&movement.bind().into_network(), NetworkMessageType::Unreliable);

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

        let block_selection = self.block_selection.clone().upcast();
        self.base_mut().add_child(block_selection);
    }

    fn process(&mut self, delta: f64) {
        let now = std::time::Instant::now();

        self.physics.step(delta as f32);

        let mut map = self.chunk_map.bind_mut();
        map.send_chunks_to_load();
        map.spawn_loaded_chunks(&self.physics);

        map.update_chunks();

        let elapsed = now.elapsed();
        if elapsed > std::time::Duration::from_millis(30) {
            log::debug!(target: "world", "World \"{}\" process: {:.2?}", self.slug, elapsed);
        }
    }
}
