use crate::client_scripts::resource_manager::ResourceManager;
use crate::console::console_handler::Console;
use crate::debug::debug_info::DebugInfo;
use crate::logger::CONSOLE_LOGGER;
use crate::network::client::NetworkContainer;
use crate::world::world_manager::WorldManager;
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

#[godot_api]
impl Main {
    fn handle_console_command(&mut self, command: String) {
        if command.len() == 0 {
            return;
        }
        NetworkContainer::send_console_command(command);
    }
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

    fn process(&mut self, delta: f64) {
        for message in Console::get_input_receiver().try_iter() {
            self.handle_console_command(message);
        }

        if let Err(e) = NetworkContainer::update(delta, self) {
            error!("Network process error: {}", e);
            Main::close();
        }
        self.debug_info
            .bind_mut()
            .update_debug(self.world_manager.bind(), &self.camera)
    }

    fn exit_tree(&mut self) {
        NetworkContainer::disconnect();
        info!("Exiting the game");
    }
}
