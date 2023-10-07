use bevy::prelude::Resource;
use log::info;
use serde_yaml::Error;
use std::collections::HashMap;
use std::env;
use std::fs;

use super::resource_instance::ResourceInstance;
use super::resource_instance::ResourceManifest;

/// resources: slug, ResourceInstance
#[derive(Resource)]
pub struct ResourceManager {
    resources: HashMap<String, ResourceInstance>,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            resources: HashMap::new(),
        }
    }

    pub fn get_resources(&self) -> &HashMap<String, ResourceInstance> {
        &self.resources
    }

    pub fn rescan_scripts(&mut self) {
        let mut path = env::current_dir().unwrap().clone();
        path.push("resources");
        let path_str = path.into_os_string().into_string().unwrap();
        info!("▼ Rescan resources folders inside: {}", path_str);

        let resource_paths = match fs::read_dir(path_str.clone()) {
            Ok(p) => p,
            Err(e) => {
                info!("□ read directory \"{}\" error: {}", path_str, e);
                return ();
            }
        };

        for resource_path in resource_paths {
            let current_path = resource_path.unwrap().path();

            let manifest_path = format!("{}/manifest.yml", current_path.display());

            let data = match fs::read_to_string(manifest_path.clone()) {
                Ok(d) => d,
                Err(e) => {
                    info!("□ error with manifest file {}: {}", manifest_path, e);
                    continue;
                }
            };

            let manifest_result: Result<ResourceManifest, Error> = serde_yaml::from_str(&data);
            let manifest = match manifest_result {
                Ok(m) => m,
                Err(e) => {
                    info!("□ error with parse manifest yaml {}: {}", manifest_path, e);
                    continue;
                }
            };

            let resource_instance = match ResourceInstance::from_manifest(&manifest, current_path.clone()) {
                Ok(i) => i,
                Err(e) => {
                    info!("□ error with resource {}: {}", current_path.display(), e);
                    continue;
                }
            };
            info!(
                "□ Resource \"{}\"; Title:\"{}\" v\"{}\" Author:\"{}\" Scripts:{}",
                resource_instance.get_slug(),
                resource_instance.get_title(),
                resource_instance.get_version(),
                resource_instance.get_autor(),
                resource_instance.get_scripts_count(),
            );
            self.resources.insert(manifest.slug, resource_instance);
        }
    }
}
