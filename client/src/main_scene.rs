use crate::client_scripts::resource_manager::ResourceManager;
use crate::console::console_handler::Console;
use crate::logger::CONSOLE_LOGGER;
use crate::network::client::NetworkContainer;
use crate::world::World;
use godot::engine::node::InternalMode;
use godot::engine::Engine;
use godot::prelude::*;
use log::{info, LevelFilter};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    #[base]
    base: Base<Node>,
    resource_manager: ResourceManager,
    world: Option<Gd<World>>,
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

    pub fn load_world(&mut self, slug: String) {
        let mut world = Gd::<World>::with_base(|base| World::create(base, slug));

        let world_name = GodotString::from("World");
        world.bind_mut().set_name(world_name.clone());

        self.base
            .add_child(world.upcast(), true, InternalMode::INTERNAL_MODE_FRONT);
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
        log::set_logger(&CONSOLE_LOGGER).unwrap();
        log::set_max_level(LevelFilter::Info);

        info!("Loading HonnyCraft version: {}", VERSION);

        if Engine::singleton().is_editor_hint() {
            return;
        }

        NetworkContainer::create_client("127.0.0.1:14191".to_string(), "Test_cl".to_string());
    }

    fn process(&mut self, delta: f64) {
        if Engine::singleton().is_editor_hint() {
            return;
        }

        for message in Console::get_input_receiver().try_iter() {
            self.handle_console_command(message);
        }

        NetworkContainer::update(delta, self);
    }

    fn exit_tree(&mut self) {
        if Engine::singleton().is_editor_hint() {
            return;
        }

        NetworkContainer::disconnect();
        info!("{}", "Exiting the game");
    }
}
