use rhai::exported_module;
use rhai::Dynamic;
use rhai::Engine;
use std::collections::HashMap;

use super::modules::main_api;
use super::resource_instance::ResourceInstance;

pub struct ResourceManager {
    rhai_engine: Engine,
    resources: HashMap<String, ResourceInstance>,

    server_target_media_count: u32,
}

impl ResourceManager {
    pub fn new() -> Self {
        let mut engine = Engine::new();

        engine.register_global_module(exported_module!(main_api).into());

        ResourceManager {
            rhai_engine: engine,
            resources: HashMap::new(),
            server_target_media_count: 0,
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

    pub fn get_resource_mut(&mut self, slug: &String) -> Option<&mut ResourceInstance> {
        self.resources.get_mut(slug)
    }

    pub fn set_target_media_count(&mut self, target: u32) {
        self.server_target_media_count = target;
    }

    pub fn get_target_media_count(&mut self) -> &u32 {
        &self.server_target_media_count
    }

    pub fn get_media_count(&self) -> u32 {
        let mut count: u32 = 0;
        for (slug, resource) in self.resources.iter() {
            count += resource.get_media_count() as u32;
        }
        return count;
    }

    pub fn _run_event(&mut self, callback_name: &String, args: &Vec<Dynamic>) {
        for (_slug, resource) in self.resources.iter_mut() {
            resource.run_event(&mut self.rhai_engine, callback_name, args);
        }
    }
}
