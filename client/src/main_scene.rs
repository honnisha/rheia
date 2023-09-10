use crate::console::console_handler::Console;
use crate::debug::debug_info::DebugInfo;
use crate::logger::CONSOLE_LOGGER;
use crate::network::client::NetworkContainer;
use crate::world::world_manager::WorldManager;
use crate::{client_scripts::resource_manager::ResourceManager, entities::position::GodotPositionConverter};
use common::network::messages::ServerMessages;
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
    resource_manager: ResourceManager,
    world_manager: Gd<WorldManager>,
    console: Gd<Console>,
    debug_info: Gd<DebugInfo>,
    camera: Gd<Camera3D>,
}

impl Main {
    pub fn get_resource_manager_mut(&mut self) -> &mut ResourceManager {
        &mut self.resource_manager
    }

    pub fn _get_world_manager(&self) -> GdRef<WorldManager> {
        self.world_manager.bind()
    }

    pub fn get_world_manager_mut(&mut self) -> GdMut<WorldManager> {
        self.world_manager.bind_mut()
    }

    pub fn close() {
        Engine::singleton().get_main_loop().unwrap().cast::<SceneTree>().quit();
    }
}

#[godot_api]
impl NodeVirtual for Main {
    fn init(base: Base<Node>) -> Self {
        let camera = load::<PackedScene>("res://scenes/camera_3d.tscn").instantiate_as::<Camera3D>();
        Main {
            base,
            resource_manager: ResourceManager::new(),
            world_manager: Gd::<WorldManager>::with_base(|base| WorldManager::create(base, &camera)),
            console: load::<PackedScene>("res://scenes/console.tscn").instantiate_as::<Console>(),
            debug_info: load::<PackedScene>("res://scenes/debug_info.tscn").instantiate_as::<DebugInfo>(),
            camera: camera,
        }
    }

    fn ready(&mut self) {
        log::set_logger(&CONSOLE_LOGGER).unwrap();
        log::set_max_level(LevelFilter::Info);

        self.base.add_child(self.world_manager.share().upcast());
        self.base.add_child(self.console.share().upcast());
        self.base.add_child(self.debug_info.share().upcast());
        self.base.add_child(self.camera.share().upcast());

        self.debug_info.bind_mut().toggle(true);

        info!("Loading HonnyCraft version: {}", VERSION);

        if let Err(e) = NetworkContainer::create_client("127.0.0.1:14191".to_string(), "Test_cl".to_string()) {
            error!("Network connection error: {}", e);
            Main::close();
        }
        NetworkContainer::spawn_network_thread();
    }

    fn process(&mut self, _delta: f64) {
        let now = std::time::Instant::now();

        for command in Console::iter_console_input() {
            NetworkContainer::send_console_command(command);
        }

        // Recieve errors from network thread
        for error in NetworkContainer::errors_iter() {
            error!("Network error: {}", error);
            Main::close();
        }

        // Recieve decoded server messages from network thread
        for decoded in NetworkContainer::server_messages_iter() {
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
                    let mut world_manager = self.get_world_manager_mut();
                    world_manager.load_chunk(world_slug, chunk_position, sections);
                }
                ServerMessages::UnloadChunks { chunks, world_slug } => {
                    self.get_world_manager_mut().unload_chunk(world_slug, chunks);
                }
                _ => panic!("unsupported chunks message"),
            }
        }

        self.debug_info
            .bind_mut()
            .update_debug(self.world_manager.bind(), &self.camera);

        let elapsed = now.elapsed();
        if elapsed > std::time::Duration::from_millis(3) {
            println!("main_scene process: {:.2?}", elapsed);
        }
    }

    fn exit_tree(&mut self) {
        NetworkContainer::disconnect();
        info!("Exiting the game");
    }
}
