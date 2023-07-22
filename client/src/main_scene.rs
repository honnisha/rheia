use std::sync::{Arc, RwLock};

use crate::console::console_handler::Console;
use crate::logger::CONSOLE_LOGGER;
use crate::network::client::NetworkContainer;
use crate::world::world_manager::WorldManager;
use crate::{client_scripts::resource_manager::ResourceManager};
use godot::engine::{Engine};
use godot::prelude::*;
use log::{error, info, LevelFilter};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    #[base]
    base: Base<Node>,
    resource_manager: ResourceManager,
    pub world_manager: WorldManager,
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

    pub fn teleport_player(&mut self, world_slug: String, location: [f32; 3]) {
        self.world_manager.teleport_player(&mut self.base, world_slug, location);
    }
}

#[godot_api]
impl NodeVirtual for Main {
    fn init(base: Base<Node>) -> Self {
        Main {
            base,
            resource_manager: ResourceManager::new(),
            world_manager: WorldManager::new(),
        }
    }

    fn ready(&mut self) {
        log::set_logger(&CONSOLE_LOGGER).unwrap();
        log::set_max_level(LevelFilter::Info);

        info!("Loading HonnyCraft version: {}", VERSION);

        if Engine::singleton().is_editor_hint() {
            return;
        }

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
                error!("Network error: {}", e);
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
