use crate::console::console_handler::Console;
use serde_yaml::Error;
use std::collections::HashMap;
use std::env;
use std::fs;

use super::resource_instance::ResourceInstance;
use super::resource_instance::ResourceManifest;

/// resources: slug, ResourceInstance
pub struct ResourceManager {
    resources: HashMap<String, ResourceInstance>,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            resources: HashMap::new(),
        }
    }

    pub fn rescan_scripts(&mut self) {
        let mut path = env::current_dir().unwrap().clone();
        path.push("resources");
        let path_str = path.into_os_string().into_string().unwrap();
        Console::send_message(format!("▼ Rescan resources folders inside: {}", path_str));

        let resource_paths = match fs::read_dir(path_str.clone()) {
            Ok(p) => p,
            Err(e) => {
                Console::send_message(format!("□ read directory \"{}\" error: {}", path_str, e));
                return ();
            }
        };

        for resource_path in resource_paths {
            let current_path = resource_path.unwrap().path();

            let manifest_path = format!("{}/manifest.yml", current_path.display());

            let data = match fs::read_to_string(manifest_path.clone()) {
                Ok(d) => d,
                Err(e) => {
                    Console::send_message(format!("□ error with manifest file {}: {}", manifest_path, e));
                    continue;
                }
            };

            let manifest_result: Result<ResourceManifest, Error> = serde_yaml::from_str(&data);
            let manifest = match manifest_result {
                Ok(m) => m,
                Err(e) => {
                    Console::send_message(format!("□ error with parse manifest yaml {}: {}", manifest_path, e));
                    continue;
                }
            };

            let resource_instance = match ResourceInstance::from_manifest(&manifest, current_path.clone()) {
                Ok(i) => i,
                Err(e) => {
                    Console::send_message(format!("□ error with resource {}: {}", current_path.display(), e));
                    continue;
                }
            };
            Console::send_message(format!(
                "□ Resource \"{}\"; Title:\"{}\" v\"{}\" Author:\"{}\" Scripts:{}",
                resource_instance.get_slug(),
                resource_instance.get_title(),
                resource_instance.get_version(),
                resource_instance.get_autor(),
                resource_instance.get_scripts_count(),
            ));
            self.resources.insert(manifest.slug, resource_instance);
        }
    }
}
