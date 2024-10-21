use bevy::prelude::Res;
use bevy::prelude::ResMut;
use bevy::prelude::Resource;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::LaunchSettings;

use super::default_resources::DEFAULT_MEDIA;
use super::resource_instance::ResourceInstance;

#[derive(Resource)]
pub struct ResourceManager {
    resources: HashMap<String, ResourceInstance>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            resources: Default::default(),
        }
    }

    pub fn get_resources(&self) -> &HashMap<String, ResourceInstance> {
        &self.resources
    }

    pub fn get_media_count(&self) -> u32 {
        let mut count: u32 = 0;
        for (_slug, resource) in self.resources.iter() {
            count += resource.get_media_count() as u32;
        }
        return count;
    }

    pub fn has_media(&self, slug: &String) -> bool {
        if DEFAULT_MEDIA.contains(&slug.as_str()) {
            return true;
        }

        let s: Vec<&str> = slug.split('/').collect();
        if s.len() < 2 {
            return false;
        }

        for (slug, resource) in self.resources.iter() {
            let res_slug = s[1..s.len()].join("/");
            if slug == s.get(0).unwrap() && resource.has_media(&res_slug) {
                return true;
            }
        }
        return false;
    }

    pub fn rescan_scripts(&mut self, path: PathBuf) {
        let path_str = path.into_os_string().into_string().unwrap();
        log::info!(target: "resources", "▼ Rescan resources folders inside: &e{}", path_str);

        let resource_paths = match fs::read_dir(path_str.clone()) {
            Ok(p) => p,
            Err(e) => {
                log::info!(target: "resources", "□ read directory &e\"{}\"&r error: &c{}", path_str, e);
                return ();
            }
        };

        for resource_path in resource_paths {
            let resource_path = resource_path.unwrap().path();

            let mut manifest_path = resource_path.clone();
            manifest_path.push("manifest.yml");
            if !manifest_path.exists() {
                continue;
            }

            let resource_instance = match ResourceInstance::from_manifest(resource_path.clone()) {
                Ok(i) => i,
                Err(e) => {
                    log::error!(target: "resources", "□ error with resource {}: &c{}", resource_path.display(), e);
                    continue;
                }
            };
            log::info!(
                target: "resources",
                "□ Resource &2\"{}\"&r successfully loaded; Title:\"{}\" v\"{}\" Author:\"{}\" Scripts:{} Media:{}",
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

pub(crate) fn rescan_scripts(mut resource_manager: ResMut<ResourceManager>, launch_settings: Res<LaunchSettings>) {
    resource_manager.rescan_scripts(launch_settings.get_resources_path());
}
