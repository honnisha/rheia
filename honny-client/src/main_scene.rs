use crate::client_scripts::resource_manager::ResourceManager;
use crate::console::console_handler::Console;
use crate::controller::player_controller::ContollerViewMode;
use crate::debug::debug_info::DebugInfo;
use crate::logger::CONSOLE_LOGGER;
use crate::network::client::{NetworkContainer, NetworkLockType};
use crate::utils::position::GodotPositionConverter;
use crate::world::world_manager::WorldManager;
use common::chunks::chunk_position::ChunkPosition;
use common::network::client::ClientNetwork;
use common::network::messages::{ClientMessages, NetworkMessageType, ServerMessages};
use common::physics::rapier::{
    RapierPhysicsCharacterController, RapierPhysicsColliderBuilder, RapierPhysicsContainer,
    RapierPhysicsRigidBodyEntity, RapierPhysicsStaticEntity,
};
use godot::engine::Engine;
use godot::prelude::*;
use log::{error, info, LevelFilter};

pub type FloatType = f32;
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub type PhysicsRigidBodyEntityType = RapierPhysicsRigidBodyEntity;
pub type PhysicsStaticEntityType = RapierPhysicsStaticEntity;
pub type PhysicsColliderBuilderType = RapierPhysicsColliderBuilder;
pub type PhysicsCharacterControllerType = RapierPhysicsCharacterController;

pub type PhysicsContainerType = RapierPhysicsContainer;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    base: Base<Node>,

    network: Option<NetworkContainer>,

    resource_manager: ResourceManager,
    world_manager: WorldManager,
    console: Gd<Console>,
    debug_info: Gd<DebugInfo>,
}

impl Main {
    fn get_network_lock(&self) -> NetworkLockType {
        self.network.as_ref().unwrap().get_network_lock()
    }

    pub fn network_send_message(&self, message: &ClientMessages, message_type: NetworkMessageType) {
        let lock = self.get_network_lock();
        let network = lock.read();
        network.send_message(message, message_type);
    }

    pub fn get_resource_manager_mut(&mut self) -> &mut ResourceManager {
        &mut self.resource_manager
    }

    pub fn _get_world_manager(&self) -> &WorldManager {
        &self.world_manager
    }

    pub fn get_world_manager_mut(&mut self) -> &mut WorldManager {
        &mut self.world_manager
    }

    pub fn close() {
        Engine::singleton().get_main_loop().unwrap().cast::<SceneTree>().quit();
    }
}

#[godot_api]
impl INode for Main {
    fn init(base: Base<Node>) -> Self {
        let world_manager = WorldManager::create(base.as_gd().clone());
        Main {
            base,
            network: None,
            resource_manager: ResourceManager::new(),
            world_manager: world_manager,
            console: load::<PackedScene>("res://scenes/console.tscn").instantiate_as::<Console>(),
            debug_info: load::<PackedScene>("res://scenes/debug_info.tscn").instantiate_as::<DebugInfo>(),
        }
    }

    fn ready(&mut self) {
        log::set_logger(&CONSOLE_LOGGER).unwrap();
        log::set_max_level(LevelFilter::Info);

        let console = self.console.clone().upcast();
        self.base_mut().add_child(console);

        let debug_info = self.debug_info.clone().upcast();
        self.base_mut().add_child(debug_info);

        self.debug_info.bind_mut().toggle(true);

        info!("Loading HonnyCraft version: {}", VERSION);

        let network = match NetworkContainer::new("127.0.0.1:19134".to_string()) {
            Ok(c) => c,
            Err(e) => {
                error!("Network connection error: {}", e);
                Main::close();
                return;
            }
        };

        self.network = Some(network);
    }

    fn process(&mut self, _delta: f64) {
        let now = std::time::Instant::now();

        let lock = self.get_network_lock();
        let network = lock.read();

        // Recieve errors from network thread
        for error in network.iter_errors() {
            error!("Network error: {}", error);
            Main::close();
        }

        for command in Console::iter_console_input() {
            let message = ClientMessages::ConsoleInput { command };
            network.send_message(&message, NetworkMessageType::ReliableOrdered);
        }

        let mut chunks: Vec<ChunkPosition> = Default::default();

        // Recieve decoded server messages from network thread
        for decoded in network.iter_server_messages() {
            match decoded {
                ServerMessages::AllowConnection => {
                    let connection_info = ClientMessages::ConnectionInfo {
                        login: "Test_cl".to_string(),
                    };
                    network.send_message(&connection_info, NetworkMessageType::ReliableOrdered);
                }
                ServerMessages::ConsoleOutput { message } => {
                    info!("{}", message);
                }
                ServerMessages::Resource { slug, scripts } => {
                    let resource_manager = self.get_resource_manager_mut();
                    info!("Start loading client resource slug:\"{}\"", slug);
                    match resource_manager.try_load(&slug, scripts) {
                        Ok(_) => {
                            info!("Client resource slug:\"{}\" loaded", slug);
                        }
                        Err(e) => {
                            error!("Client resource slug:\"{}\" error: {}", slug, e);
                        }
                    }
                }
                ServerMessages::Teleport {
                    world_slug,
                    location,
                    yaw,
                    pitch,
                } => {
                    self.get_world_manager_mut().teleport_player(
                        world_slug,
                        GodotPositionConverter::vector_gd_from_network(&location),
                        yaw,
                        pitch,
                    );
                }
                ServerMessages::ChunkSectionInfo {
                    world_slug,
                    chunk_position,
                    sections,
                } => {
                    let world_manager = self.get_world_manager_mut();
                    world_manager.load_chunk(world_slug, chunk_position, sections);
                    chunks.push(chunk_position);
                }
                ServerMessages::UnloadChunks { chunks, world_slug } => {
                    self.get_world_manager_mut().unload_chunk(world_slug, chunks);
                }
                _ => panic!("unsupported chunks message"),
            }
        }

        if chunks.len() > 0 {
            let input = ClientMessages::ChunkRecieved {
                chunk_positions: chunks,
            };
            network.send_message(&input, NetworkMessageType::ReliableOrdered);
        }

        self.debug_info.bind_mut().update_debug(&self.world_manager, network);

        let input = Input::singleton();
        if input.is_action_just_pressed("ui_toggle_console".into()) {
            self.console.bind_mut().toggle(!Console::is_active());

            if Console::is_active() {
                self.debug_info.bind_mut().toggle(false);
            }
        }
        if input.is_action_just_pressed("ui_toggle_debug".into()) {
            self.debug_info.bind_mut().toggle(!DebugInfo::is_active());

            if DebugInfo::is_active() {
                self.console.bind_mut().toggle(false);
            }
        }
        if input.is_action_just_pressed("ui_view_mode_switch".into()) {
            if let Some(w) = self.world_manager.get_world_mut() {
                let mut w = w.bind_mut();
                let mut player_controller = w.get_player_controller_mut().bind_mut();
                let view_mode = player_controller.get_view_mode();
                match view_mode {
                    ContollerViewMode::FirstPersonView => {
                        player_controller.set_view_mode(ContollerViewMode::ThirdPersonView);
                    }
                    ContollerViewMode::ThirdPersonView => {
                        player_controller.set_view_mode(ContollerViewMode::FirstPersonView);
                    }
                }
            }
        }

        let elapsed = now.elapsed();
        if elapsed > std::time::Duration::from_millis(20) {
            println!("Main process: {:.2?}", elapsed);
        }
    }

    fn exit_tree(&mut self) {
        if let Some(n) = self.network.as_ref() {
            n.disconnect();
        }
        info!("Exiting the game");
    }
}
