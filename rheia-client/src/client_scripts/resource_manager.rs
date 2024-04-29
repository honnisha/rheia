use rhai::exported_module;
use rhai::Dynamic;
use rhai::Engine;
use std::collections::HashMap;

use super::modules::main_api;
use super::resource_instance::ResourceInstance;

pub struct ResourceManager {
    rhai_engine: Engine,
    resources: HashMap<String, ResourceInstance>,
}

impl ResourceManager {
    pub fn new() -> Self {
        let mut engine = Engine::new();

        engine.register_global_module(exported_module!(main_api).into());

        ResourceManager {
            rhai_engine: engine,
            resources: HashMap::new(),
        }
    }

    pub fn try_load(&mut self, slug: &String, scripts: HashMap<String, String>) -> Result<(), String> {
        match ResourceInstance::try_init(&mut self.rhai_engine, slug, scripts) {
            Ok(i) => self.resources.insert(slug.clone(), i),
            Err(e) => {
                return Err(e);
            }
        };
        Ok(())
    }

    pub fn run_event(&mut self, callback_name: &String, args: &Vec<Dynamic>) {
        for (_slug, resource) in self.resources.iter_mut() {
            resource.run_event(&mut self.rhai_engine, callback_name, args);
        }
    }
}
