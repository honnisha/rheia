use crate::console::console_handler::Console;
use crate::debug::debug_info::DebugInfo;
use crate::entities::position::GodotPositionConverter;
use crate::logger::CONSOLE_LOGGER;
use crate::network::client::{NetworkContainer, NetworkLockType};
use crate::world::world_manager::WorldManager;
use crate::{client_scripts::resource_manager::ResourceManager};
use common::chunks::chunk_position::ChunkPosition;
use common::network::client::ClientNetwork;
use common::network::messages::{ClientMessages, NetworkMessageType, ServerMessages};
use godot::engine::Engine;
use godot::prelude::*;
use log::{error, info, LevelFilter};

pub const CHUNKS_DISTANCE: u16 = 16;

pub type FloatType = f32;
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    #[base]
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
impl NodeVirtual for Main {
    fn init(base: Base<Node>) -> Self {
        let world_manager = WorldManager::create(base.share());
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

        self.base.add_child(self.console.share().upcast());
        self.base.add_child(self.debug_info.share().upcast());

        self.debug_info.bind_mut().toggle(true);

        info!("Loading HonnyCraft version: {}", VERSION);

        let network = match NetworkContainer::new("127.0.0.1:14191".to_string(), "Test_cl".to_string()) {
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
                        GodotPositionConverter::vec3_from_array(&location),
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
                    // println!("load_chunk {}", chunk_position);
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

        self.debug_info
            .bind_mut()
            .update_debug(&self.world_manager, network);

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
