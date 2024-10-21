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

    pub fn try_load(&mut self, slug: &String, scripts: HashMap<String, String>, is_network: bool) -> Result<(), String> {
        match ResourceInstance::try_init(&mut self.rhai_engine, slug, scripts, is_network) {
            Ok(resource_instance) => {
                self.resources.insert(slug.clone(), resource_instance);
            }
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

    pub fn get_media_count(&self, only_network: bool) -> u32 {
        let mut count: u32 = 0;
        for (_slug, resource) in self.resources.iter() {
            if !only_network || resource.is_network() {
                count += resource.get_media_count() as u32;
            }
        }
        return count;
    }

    pub fn has_media(&self, slug: &String) -> bool {
        for (_slug, resource) in self.resources.iter() {
            if resource.has_media(slug) {
                return true;
            }
        }
        return false;
    }

    pub fn _run_event(&mut self, callback_name: &String, args: &Vec<Dynamic>) {
        for (_slug, resource) in self.resources.iter_mut() {
            resource._run_event(&mut self.rhai_engine, callback_name, args);
        }
    }
}
