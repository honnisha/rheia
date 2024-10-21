use rhai::exported_module;
use rhai::Dynamic;
use rhai::Engine;
use std::collections::HashMap;

use super::local_loader::get_local_resources;
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
        if self.resources.contains_key(slug) {
            return Err(format!("Resource \"{}\" already exists", slug));
        }

        match ResourceInstance::try_init(&mut self.rhai_engine, slug, scripts, is_network) {
            Ok(resource_instance) => {
                self.resources.insert(slug.clone(), resource_instance);
            }
            Err(e) => return Err(e)
        };
        Ok(())
    }

    pub fn load_local_resources(&mut self) -> Result<(), String> {
        let local_resources = match get_local_resources() {
            Ok(m) => m,
            Err(e) => return Err(e),
        };
        for mut local_resource in local_resources {
            match self.try_load(&local_resource.slug, local_resource.scripts, false) {
                Ok(_) => (),
                Err(e) => return Err(e)
            }

            let resource_instance = self.resources.get_mut(&local_resource.slug).unwrap();

            for (media_slug, media_data) in local_resource.media.drain() {
                resource_instance.add_media(media_slug, media_data);
            }

            log::info!(target:"resources", "Local resource \"{}\" loaded", local_resource.slug);
        }
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

    pub fn get_resources_count(&mut self) -> usize {
        self.resources.len()
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
        let s: Vec<&str> = slug.split("://").collect();
        if s.len() < 2 {
            return false;
        }

        for (resource_slug, resource) in self.resources.iter() {
            let res_slug = s[1..s.len()].join("/");

            if resource_slug == s.get(0).unwrap() && resource.has_media(&res_slug) {
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
