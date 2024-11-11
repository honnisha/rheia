use super::{
    chunks::chunks_map::ChunkMap,
    physics::PhysicsProxy,
    worlds_manager::{BlockStorageType, TextureMapperType},
};
use crate::{
    controller::{entity_movement::EntityMovement, player_controller::PlayerController},
    entities::entities_manager::EntitiesManager,
    scenes::main_scene::MainScene,
    utils::{bridge::IntoChunkPositionVector, primitives::generate_lines},
};
use godot::{classes::Material, prelude::*};
use network::messages::NetworkMessageType;

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
        let mut physics = PhysicsProxy::default();
        let mut chunk_map = Gd::<ChunkMap>::from_init_fn(|base| ChunkMap::create(base));
        chunk_map.bind_mut().base_mut().set_name("ChunkMap");
        let player_controller =
            Gd::<PlayerController>::from_init_fn(|base| PlayerController::create(base, &mut physics));

        let mut selection = Node3D::new_alloc();

        let positions = vec![
            Vector3::new(-0.5, -0.5, -0.5),
            Vector3::new(0.5, -0.5, -0.5),
            Vector3::new(0.5, -0.5, -0.5),
            Vector3::new(0.5, 0.5, -0.5),
            Vector3::new(0.5, 0.5, -0.5),
            Vector3::new(-0.5, 0.5, -0.5),
            Vector3::new(-0.5, 0.5, -0.5),
            Vector3::new(-0.5, -0.5, -0.5),
        ];
        let mesh = generate_lines(positions, Color::from_rgb(0.0, 0.0, 0.0));
        selection.add_child(&mesh);
        selection.set_visible(false);

        Self {
            base,
            slug: slug,
            chunk_map,

            physics,
            player_controller,

            entities_manager: Gd::<EntitiesManager>::from_init_fn(|base| EntitiesManager::create(base)),

            block_selection: selection,

            texture_mapper,
            material,
            block_storage,
        }
    }

    pub fn get_block_selection_mut(&mut self) -> &mut Gd<Node3D> {
        &mut self.block_selection
    }

    pub fn _get_entities_manager(&self) -> GdRef<EntitiesManager> {
        self.entities_manager.bind()
    }

    pub fn get_entities_manager_mut(&mut self) -> GdMut<EntitiesManager> {
        self.entities_manager.bind_mut()
    }

    pub fn get_physics_mut(&mut self) -> &mut PhysicsProxy {
        &mut self.physics
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

    pub fn get_main(&self) -> Gd<MainScene> {
        let main = self
            .base()
            .to_godot()
            .get_parent()
            .expect("main scene not found")
            .cast::<MainScene>();
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
        let chunk_map = self.chunk_map.clone();
        self.base_mut().add_child(&chunk_map);

        // Bind world player move signal
        let obj = self.base().to_godot().clone();
        self.player_controller.bind_mut().base_mut().connect(
            "on_player_move",
            &Callable::from_object_method(&obj, "handler_player_move"),
        );

        let player_controller = self.player_controller.clone();
        self.base_mut().add_child(&player_controller);

        let entities_manager = self.entities_manager.clone();
        self.base_mut().add_child(&entities_manager);

        let block_selection = self.block_selection.clone();
        self.base_mut().add_child(&block_selection);
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
