use std::collections::HashMap;

use godot::{
    classes::{file_access::ModeFlags, DirAccess, FileAccess, Resource, ResourceLoader},
    obj::Gd,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct LocalResourceManifest {
    pub slug: String,
    pub client_scripts: Option<Vec<String>>,
    pub media: Option<Vec<String>>,
}

pub(crate) struct LocalResource {
    pub slug: String,
    pub scripts: HashMap<String, String>,
    pub media: HashMap<String, Gd<Resource>>,
}

pub(crate) fn get_local_resources() -> Result<Vec<LocalResource>, String> {
    let mut result: Vec<LocalResource> = Default::default();

    for dir in DirAccess::get_directories_at("res://assets/resources").as_slice() {
        let manifest_path = format!("res://assets/resources/{}/manifest.json", dir);
        let Some(manifest_file) = FileAccess::open(&manifest_path, ModeFlags::READ) else {
            return Err(format!("Manifest {} file error", manifest_path));
        };

        let manifest_text: String = manifest_file.get_as_text().into();
        let manifest_result: Result<LocalResourceManifest, serde_yaml::Error> = serde_yaml::from_str(&manifest_text);
        let manifest = match manifest_result {
            Ok(m) => m,
            Err(e) => {
                return Err(format!("Manifest {} error: {}", manifest_path, e));
            }
        };
        let mut resource = LocalResource {
            slug: manifest.slug.clone(),
            scripts: Default::default(),
            media: Default::default(),
        };

        let mut resource_loader = ResourceLoader::singleton();
        if let Some(client_scripts) = manifest.client_scripts {
            for script_path in client_scripts {
                let _file_resource = match resource_loader.load(&script_path) {
                    Some(r) => r,
                    None => {
                        return Err(format!(
                            "resource \"{}\" ResourceLoader cannot find {} file",
                            resource.slug, script_path
                        ))
                    }
                };
                unimplemented!();
                //resource.scripts.insert(script_path.replace("res://", ""), script_text);
            }
        }

        if let Some(media_list) = manifest.media {
            for media_path in media_list {
                let file_resource = match resource_loader.load(&media_path) {
                    Some(r) => r,
                    None => {
                        return Err(format!(
                            "resource \"{}\" ResourceLoader cannot find {} file",
                            resource.slug, media_path
                        ))
                    }
                };
                resource.media.insert(media_path.replace("res://", ""), file_resource);
            }
        }
        result.push(resource);
    }

    return Ok(result);
}
