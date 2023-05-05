use std::sync::{Arc, Mutex, MutexGuard};

use crate::client_scripts::resource_manager::ResourceManager;
use crate::console::console_handler::Console;
use crate::network::client::NetworkClient;
use crate::world::World;
use godot::engine::Engine;
use godot::engine::node::InternalMode;
use godot::prelude::*;
use lazy_static::lazy_static;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    #[base]
    base: Base<Node>,
    resource_manager: ResourceManager,
    world: Option<Gd<World>>,
}

lazy_static! {
    static ref NETWORK_CLIENT: Arc<Mutex<NetworkClient>> = Arc::new(Mutex::new(NetworkClient::init()));
}

#[godot_api]
impl Main {
    fn handle_console_command(&mut self, command: String) {
        if command.len() == 0 {
            return;
        }
        Main::get_client().send_console_command(command);
    }

    pub fn get_client() -> MutexGuard<'static, NetworkClient> {
        NETWORK_CLIENT.lock().unwrap()
    }
}

impl Main {
    pub fn get_resource_manager_mut(&mut self) -> &mut ResourceManager {
        &mut self.resource_manager
    }

    pub fn load_world(&mut self, slug: String) {
        let mut world = Gd::<World>::with_base(|base| World::create(base, slug));

        let world_name = GodotString::from("World");
        world.bind_mut().set_name(world_name.clone());

        self.base.add_child(world.upcast(), true, InternalMode::INTERNAL_MODE_FRONT);
        self.world = Some(self.base.get_node_as::<World>(world_name));

        godot_print!("World \"{}\" loaded;", self.world.as_ref().unwrap().bind().get_slug());
    }
}

#[godot_api]
impl NodeVirtual for Main {
    fn init(base: Base<Node>) -> Self {
        Main {
            base,
            resource_manager: ResourceManager::new(),
            world: None,
        }
    }

    fn ready(&mut self) {
        godot_print!("Loading HonnyCraft version: {}", VERSION);

        if Engine::singleton().is_editor_hint() {
            return;
        }

        NETWORK_CLIENT.lock().unwrap().create_client(
            "127.0.0.1:14191".to_string(),
            "TestUser".to_string(),
        );
    }

    fn process(&mut self, delta: f64) {
        if Engine::singleton().is_editor_hint() {
            return;
        }

        for message in Console::get_input_receiver().try_iter() {
            self.handle_console_command(message);
        }

        Main::get_client().update(delta, self);
    }

    fn exit_tree(&mut self) {
        if Engine::singleton().is_editor_hint() {
            return;
        }

        Main::get_client().disconnect();
    }
}
