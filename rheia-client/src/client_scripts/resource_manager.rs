use common::utils::calculate_hash;
use common::utils::split_resource_path;
use network::messages::ResurceScheme;
use rhai::exported_module;
use rhai::Dynamic;
use rhai::Engine;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Read;
use std::rc::Rc;

use super::local_loader::get_local_resources;
use super::modules::main_api;
use super::resource_instance::MediaResource;
use super::resource_instance::ResourceInstance;

pub struct ResourceManager {
    rhai_engine: Rc<RefCell<Engine>>,
    resources: HashMap<String, ResourceInstance>,

    resources_scheme: Option<Vec<ResurceScheme>>,
    archive_data: Option<Vec<u8>>,
    archive_hash: Option<u64>,
}

impl ResourceManager {
    pub fn new() -> Self {
        let mut engine = Engine::new();

        engine.register_global_module(exported_module!(main_api).into());

        ResourceManager {
            rhai_engine: Rc::new(RefCell::new(engine)),
            resources: HashMap::new(),

            resources_scheme: Default::default(),
            archive_hash: Default::default(),
            archive_data: Default::default(),
        }
    }

    pub fn set_resource_scheme(&mut self, list: Vec<ResurceScheme>, archive_hash: u64) {
        self.resources_scheme = Some(list);
        self.archive_hash = Some(archive_hash);
    }

    pub fn _get_resource(&self, slug: &String) -> Option<&ResourceInstance> {
        self.resources.get(slug)
    }

    pub fn get_resource_mut(&mut self, slug: &String) -> Option<&mut ResourceInstance> {
        self.resources.get_mut(slug)
    }

    pub fn load_archive_chunk(&mut self, data: &mut Vec<u8>) {
        if self.archive_data.is_none() {
            self.archive_data = Some(Default::default());
        }

        self.archive_data
            .as_mut()
            .expect("archive_data is not set")
            .append(data);
    }

    pub fn load_archive(&mut self) -> Result<(), String> {
        let archive_data = self.archive_data.take().expect("archive_data is not set");

        let archive_hash = calculate_hash(&archive_data);
        let original_archive_hash = self.archive_hash.as_ref().expect("archive_hash is None");
        if *original_archive_hash != archive_hash {
            return Err(format!(
                "archive_data hash {} != original {}",
                archive_hash, original_archive_hash
            ));
        }

        let resources_scheme = self.resources_scheme.take().expect("resources_scheme is not set");

        for resource_scheme in resources_scheme.iter() {
            let resource = ResourceInstance::new(resource_scheme.slug.clone(), true);
            self.add_resource(resource);
        }

        let rhai_engine = self.rhai_engine.clone();

        let file = std::io::Cursor::new(&archive_data);
        let mut zip = zip::ZipArchive::new(file).unwrap();
        for i in 0..zip.len() {
            let mut file = zip.by_index(i).unwrap();

            for resource_scheme in resources_scheme.iter() {
                let script_name = resource_scheme.scripts.get(file.name());
                let media_name = resource_scheme.media.get(file.name());

                // Load rhai scripts
                if script_name.is_some() {
                    let resource = self.get_resource_mut(&resource_scheme.slug).unwrap();

                    let mut code = String::new();
                    file.read_to_string(&mut code).unwrap();
                    resource.add_script(&mut rhai_engine.borrow_mut(), script_name.unwrap().to_string(), code)?;
                }
                // Load media
                else if media_name.is_some() {
                    let resource = self.get_resource_mut(&resource_scheme.slug).unwrap();

                    let mut data = Vec::new();
                    file.read(&mut data).unwrap();

                    let hash = calculate_hash(&data);

                    if hash.to_string() != file.name() {
                        return Err(format!(
                            "file \"{}\" network hash {} != original {}",
                            media_name.unwrap().to_string(),
                            hash.to_string(),
                            file.name(),
                        ));
                    }

                    if let Err(e) = resource.add_media_from_bytes(media_name.unwrap().to_string(), data) {
                        return Err(format!(
                            "file \"{}\" loading error: {}",
                            media_name.unwrap().to_string(),
                            e
                        ));
                    }
                } else {
                    return Err(format!("File from archive \"{}\" is not found in schema", file.name()));
                }
            }
        }

        Ok(())
    }

    pub fn add_resource(&mut self, resource: ResourceInstance) {
        self.resources.insert(resource.get_slug().clone(), resource);
    }

    pub fn load_local_resources(&mut self) -> Result<(), String> {
        let local_resources = match get_local_resources() {
            Ok(m) => m,
            Err(e) => return Err(e),
        };
        for mut local_resource in local_resources {
            let mut resource_instance = ResourceInstance::new(local_resource.slug.clone(), false);

            for (script_slug, script_code) in local_resource.scripts.drain() {
                resource_instance.add_script(&mut self.rhai_engine.borrow_mut(), script_slug, script_code)?;
            }

            for (media_slug, media_data) in local_resource.media.drain() {
                if let Err(e) = resource_instance.add_media_from_resource(media_slug, media_data) {
                    return Err(e);
                }
            }

            self.add_resource(resource_instance);
        }
        Ok(())
    }

    pub fn get_resources_count(&self) -> usize {
        self.resources.len()
    }

    pub fn _get_media_count(&self, only_network: bool) -> u32 {
        let mut count: u32 = 0;
        for (_slug, resource) in self.resources.iter() {
            if !only_network || resource.is_network() {
                count += resource.get_media_count() as u32;
            }
        }
        return count;
    }

    pub fn has_media(&self, path: &String) -> bool {
        let Some((res_slug, res_path)) = split_resource_path(path) else {
            return false;
        };

        for (resource_path, resource) in self.resources.iter() {
            if *resource_path == res_slug && resource.has_media(&res_path) {
                return true;
            }
        }
        return false;
    }

    pub fn get_media(&self, path: &String) -> Option<&MediaResource> {
        let Some((res_slug, res_path)) = split_resource_path(path) else {
            return None;
        };

        for (resource_path, resource) in self.resources.iter() {
            if *resource_path == res_slug {
                return resource.get_media(&res_path);
            }
        }
        return None;
    }

    pub fn _run_event(&mut self, callback_name: &String, args: &Vec<Dynamic>) {
        for (_slug, resource) in self.resources.iter_mut() {
            resource._run_event(&mut self.rhai_engine.borrow_mut(), callback_name, args);
        }
    }
}
