use common::utils::calculate_hash;
use common::utils::split_resource_path;
use network::messages::ResurceScheme;
use parking_lot::lock_api::RwLockReadGuard;
use parking_lot::lock_api::RwLockWriteGuard;
use parking_lot::RwLock;
use rhai::exported_module;
use rhai::Dynamic;
use rhai::Engine;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Read;
use std::rc::Rc;
use std::sync::Arc;

use super::local_loader::get_local_resources;
use super::modules::main_api;
use super::resource_instance::MediaResource;
use super::resource_instance::ResourceInstance;

pub struct ResourceStorage {
    resources: HashMap<String, ResourceInstance>,
}

unsafe impl Send for ResourceStorage {}
unsafe impl Sync for ResourceStorage {}

impl Default for ResourceStorage {
    fn default() -> Self {
        Self {
            resources: Default::default(),
        }
    }
}

impl ResourceStorage {
    pub fn _get_resource(&self, slug: &String) -> Option<&ResourceInstance> {
        self.resources.get(slug)
    }

    pub fn get_resource_mut(&mut self, slug: &String) -> Option<&mut ResourceInstance> {
        self.resources.get_mut(slug)
    }

    pub fn add_resource(&mut self, resource: ResourceInstance) {
        self.resources.insert(resource.get_slug().clone(), resource);
    }

    pub fn get_resources_count(&self) -> usize {
        self.resources.len()
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

    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<'_, String, ResourceInstance> {
        self.resources.iter_mut()
    }
}

pub type ResourceStorageType = Arc<RwLock<ResourceStorage>>;

pub struct ResourceManager {
    rhai_engine: Rc<RefCell<Engine>>,
    resources_storage: ResourceStorageType,

    resources_scheme: Option<Vec<ResurceScheme>>,
    archive_data: Option<Vec<u8>>,
    archive_hash: Option<u64>,
}

enum ResourceType {
    Script { resource_slug: String, name: String },
    Media { resource_slug: String, name: String },
    None,
}

impl Default for ResourceManager {
    fn default() -> Self {
        let mut engine = Engine::new();

        engine.register_global_module(exported_module!(main_api).into());

        Self {
            rhai_engine: Rc::new(RefCell::new(engine)),
            resources_storage: Arc::new(RwLock::new(ResourceStorage::default())),

            resources_scheme: Default::default(),
            archive_hash: Default::default(),
            archive_data: Default::default(),
        }
    }
}

impl ResourceManager {
    pub fn get_resources_storage_lock(&self) -> ResourceStorageType {
        self.resources_storage.clone()
    }

    pub fn get_resources_storage(&self) -> RwLockReadGuard<'_, parking_lot::RawRwLock, ResourceStorage> {
        self.resources_storage.read()
    }

    pub fn get_resources_storage_mut(&self) -> RwLockWriteGuard<'_, parking_lot::RawRwLock, ResourceStorage> {
        self.resources_storage.write()
    }

    pub fn set_resource_scheme(&mut self, list: Vec<ResurceScheme>, archive_hash: u64) {
        self.resources_scheme = Some(list);
        self.archive_hash = Some(archive_hash);
    }

    pub fn get_resource_scheme_count(&mut self) -> (usize, usize) {
        let mut scripts_count: usize = 0;
        let mut media_count: usize = 0;
        for scheme in self.resources_scheme.as_ref().expect("resources_scheme is required") {
            scripts_count += scheme.scripts.len();
            media_count += scheme.media.len();
        }
        return (scripts_count, media_count);
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

    fn get_resource_type(resources_scheme: &Vec<ResurceScheme>, file_hash: &String) -> ResourceType {
        for resource_scheme in resources_scheme.iter() {
            let script_name = resource_scheme.scripts.get(file_hash);
            let media_name = resource_scheme.media.get(file_hash);

            if script_name.is_some() {
                let name = *script_name.as_ref().unwrap();
                return ResourceType::Script {
                    name: name.clone(),
                    resource_slug: resource_scheme.slug.clone(),
                };
            } else if media_name.is_some() {
                let name = *media_name.as_ref().unwrap();
                return ResourceType::Media {
                    name: name.clone(),
                    resource_slug: resource_scheme.slug.clone(),
                };
            }
        }
        ResourceType::None
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
            self.get_resources_storage_mut().add_resource(resource);
        }

        let rhai_engine = self.rhai_engine.clone();

        let mut resources_storage = self.get_resources_storage_mut();

        let file = std::io::Cursor::new(&archive_data);
        let mut zip = zip::ZipArchive::new(file).unwrap();
        for i in 0..zip.len() {
            let mut archive_file = zip.by_index(i).unwrap();
            let file_hash = archive_file.name().to_string();

            match ResourceManager::get_resource_type(&resources_scheme, &file_hash) {
                ResourceType::Script { name, resource_slug } => {
                    let resource = resources_storage.get_resource_mut(&resource_slug).unwrap();

                    let mut code = String::new();
                    archive_file.read_to_string(&mut code).unwrap();
                    resource.add_script(&mut rhai_engine.borrow_mut(), name, code)?;
                }
                ResourceType::Media { name, resource_slug } => {
                    let resource = resources_storage.get_resource_mut(&resource_slug).unwrap();

                    let mut archive_file_data = Vec::new();
                    for i in archive_file.bytes() {
                        archive_file_data.push(i.unwrap());
                    }

                    let hash = calculate_hash(&archive_file_data);

                    if hash.to_string() != file_hash {
                        return Err(format!(
                            "file \"{}\" network hash {} != original {}",
                            name,
                            hash.to_string(),
                            file_hash,
                        ));
                    }

                    if let Err(e) = resource.add_media_from_bytes(name.clone(), archive_file_data) {
                        return Err(format!("file \"{}\" loading error: {}", name, e));
                    }
                }
                ResourceType::None => {
                    return Err(format!("File from archive \"{}\" is not found in schema", file_hash));
                }
            }
        }

        Ok(())
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

            self.get_resources_storage_mut().add_resource(resource_instance);
        }
        Ok(())
    }

    pub fn _run_event(&mut self, callback_name: &String, args: &Vec<Dynamic>) {
        let rc = self.rhai_engine.clone();
        let mut rhai_engine = rc.borrow_mut();
        for (_slug, resource) in self.get_resources_storage_mut().iter_mut() {
            resource._run_event(&mut rhai_engine, callback_name, args);
        }
    }
}
