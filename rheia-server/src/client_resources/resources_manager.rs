use bevy::prelude::Res;
use bevy::prelude::ResMut;
use bevy::prelude::Resource;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

use crate::ServerSettings;

use super::resource_instance::ResourceInstance;

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

    pub fn rescan_scripts(&mut self, path: PathBuf) {
        let path_str = path.into_os_string().into_string().unwrap();
        log::info!(target: "resources", "▼ Rescan resources folders inside: {}", path_str);

        let resource_paths = match fs::read_dir(path_str.clone()) {
            Ok(p) => p,
            Err(e) => {
                log::info!(target: "resources", "□ read directory \"{}\" error: {}", path_str, e);
                return ();
            }
        };

        for resource_path in resource_paths {
            let current_path = resource_path.unwrap().path();

            let resource_instance = match ResourceInstance::from_manifest(current_path.clone()) {
                Ok(i) => i,
                Err(e) => {
                    log::info!(target: "resources", "□ error with resource {}: {}", current_path.display(), e);
                    continue;
                }
            };
            log::info!(
                target: "resources",
                "□ Resource \"{}\" successfully loaded; Title:\"{}\" v\"{}\" Author:\"{}\" Scripts:{} Media:{}",
                resource_instance.get_slug(),
                resource_instance.get_title(),
                resource_instance.get_version(),
                resource_instance.get_autor(),
                resource_instance.get_scripts_count(),
                resource_instance.get_media_count(),
            );
            self.resources
                .insert(resource_instance.get_slug().clone(), resource_instance);
        }
    }
}

pub(crate) fn rescan_scripts(mut resource_manager: ResMut<ResourceManager>, settings: Res<ServerSettings>) {
    let resources_path = match settings.args.resources_path.as_ref() {
        Some(p) => PathBuf::from(shellexpand::tilde(p).to_string()),
        None => {
            let mut path = env::current_dir().unwrap().clone();
            path.push("resources");
            path
        }
    };
    resource_manager.rescan_scripts(resources_path);
}
