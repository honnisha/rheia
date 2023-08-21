use crate::client_scripts::resource_manager::ResourceManager;
use crate::console::console_handler::Console;
use crate::logger::CONSOLE_LOGGER;
use crate::network::client::NetworkContainer;
use crate::world::world_manager::WorldManager;
use godot::engine::Engine;
use godot::prelude::*;
use log::{error, info, LevelFilter};

pub type FloatType = f32;
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    #[base]
    base: Base<Node>,
    resource_manager: ResourceManager,
    world_manager: Option<Gd<WorldManager>>,
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

    pub fn close() {
        Engine::singleton().get_main_loop().unwrap().cast::<SceneTree>().quit();
    }

    pub fn teleport_player(&mut self, world_slug: String, location: [FloatType; 3]) {
        self.get_world_manager_mut().teleport_player(world_slug, location);
    }

    pub fn _get_world_manager(&self) -> GdRef<WorldManager> {
        match self.world_manager.as_ref() {
            Some(w) => w.bind(),
            None => panic!("WorldManager must be loaded"),
        }
    }

    pub fn get_world_manager_mut(&mut self) -> GdMut<WorldManager> {
        match self.world_manager.as_mut() {
            Some(w) => w.bind_mut(),
            None => panic!("WorldManager must be loaded"),
        }
    }

    pub fn create_world_manager(&mut self) -> Gd<WorldManager> {
        let mut entity = Gd::<WorldManager>::with_base(|base| WorldManager::create(base));

        let name = GodotString::from("WorldManager");
        entity.bind_mut().set_name(name.clone());

        self.base.add_child(entity.upcast());
        self.base.get_node_as::<WorldManager>(name)
    }
}

#[godot_api]
impl NodeVirtual for Main {
    fn init(base: Base<Node>) -> Self {
        Main {
            base,
            resource_manager: ResourceManager::new(),
            world_manager: None,
        }
    }

    fn ready(&mut self) {
        if Engine::singleton().is_editor_hint() {
            return;
        }

        log::set_logger(&CONSOLE_LOGGER).unwrap();
        log::set_max_level(LevelFilter::Info);

        info!("Loading HonnyCraft version: {}", VERSION);

        self.world_manager = Some(self.create_world_manager());

        match NetworkContainer::create_client("127.0.0.1:14191".to_string(), "Test_cl".to_string()) {
            Ok(_) => {}
            Err(e) => {
                error!("Network connection error: {}", e);
                Main::close();
            }
        };
    }

    fn process(&mut self, delta: f64) {
        if Engine::singleton().is_editor_hint() {
            return;
        }

        for message in Console::get_input_receiver().try_iter() {
            self.handle_console_command(message);
        }

        match NetworkContainer::update(delta, self) {
            Ok(_) => {}
            Err(e) => {
                error!("Network process error: {}", e);
                Main::close();
            }
        }
    }

    fn exit_tree(&mut self) {
        if Engine::singleton().is_editor_hint() {
            return;
        }

        NetworkContainer::disconnect();
        info!("{}", "Exiting the game");
    }
}
