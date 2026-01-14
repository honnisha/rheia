use std::collections::HashMap;

use common::{ default_resources::DEFAULT_RESOURCES};
use godot::{
    classes::{file_access::ModeFlags, DirAccess, FileAccess, Resource, ResourceLoader},
    obj::{Gd, Singleton},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct LocalResourceManifest {
    pub slug: String,
    pub client_scripts: Option<Vec<String>>,
    pub media: Option<Vec<String>>,
}

#[derive(Default)]
pub(crate) struct LocalResource {
    pub slug: String,
    pub scripts: HashMap<String, String>,
    pub media: HashMap<String, Gd<Resource>>,
}

pub(crate) fn get_local_resources() -> Result<Vec<LocalResource>, String> {
    let mut result: Vec<LocalResource> = Default::default();

    for dir in DirAccess::get_directories_at("res://assets/resources").as_slice() {
        let manifest_path = format!("res://assets/resources/{}/manifest.yml", dir);
        let Some(manifest_file) = FileAccess::open(&manifest_path, ModeFlags::READ) else {
            return Err(format!(
                "&cResource &4\"{}\" &cmanifest &4\"{}\" &cis not found",
                dir, manifest_path
            ));
        };

        let manifest_text: String = manifest_file.get_as_text().into();
        let manifest_result: Result<LocalResourceManifest, serde_yaml::Error> = serde_yaml::from_str(&manifest_text);
        let manifest = match manifest_result {
            Ok(m) => m,
            Err(e) => {
                return Err(format!(
                    "&cResource &4\"{}\" &cmanifest &4\"{}\" &cerror: &4{}",
                    dir, manifest_path, e
                ));
            }
        };

        let mut resource = LocalResource {
            slug: manifest.slug.clone(),
            ..Default::default()
        };

        let mut resource_loader = ResourceLoader::singleton();
        if let Some(client_scripts) = manifest.client_scripts {
            for script_path in client_scripts {
                let script_path = format!("res://assets/resources/{}/{}", dir, script_path);
                let _file_resource = match resource_loader.load(&script_path) {
                    Some(r) => r,
                    None => {
                        return Err(format!(
                            "&cresource &4\"{}\" &cResourceLoader cannot find &4\"{}\" &cfile",
                            resource.slug, script_path
                        ));
                    }
                };
                unimplemented!();
                //resource.scripts.insert(script_path.replace("res://", ""), script_text);
            }
        }

        let mut manifest_media = match manifest.media {
            Some(s) => s,
            None => Default::default(),
        };

        if resource.slug == "default" {
            for media in DEFAULT_RESOURCES {
                if media.contains("default://") {
                    let mut media: String = (*media).into();
                    media = media.replace("default://", "res://");
                    manifest_media.push(media);
                }
            }
        }

        for media_path in manifest_media {
            let media_path = if media_path.contains("://") {
                media_path
            } else {
                format!("res://assets/resources/{}/{}", dir, media_path)
            };
            let Some(file_resource) = resource_loader.load(&media_path) else {
                return Err(format!(
                    "&cresource &4\"{}\" &cResourceLoader cannot find &4\"{}\" &cfile",
                    resource.slug, media_path
                ));
            };
            resource.media.insert(media_path.replace("res://", ""), file_resource);
        }
        result.push(resource);
    }

    return Ok(result);
}
