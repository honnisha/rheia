use std::collections::HashMap;

use godot::engine::{file_access::ModeFlags, DirAccess, FileAccess};
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
    pub media: HashMap<String, Vec<u8>>,
}

pub(crate) fn get_local_resources() -> Result<Vec<LocalResource>, String> {
    let mut result: Vec<LocalResource> = Default::default();

    for dir in DirAccess::get_directories_at("res://assets/resources".into()).as_slice() {
        let manifest_path = format!("res://assets/resources/{}/manifest.json", dir);
        let Some(manifest_file) = FileAccess::open(manifest_path.clone().into(), ModeFlags::READ) else {
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

        if let Some(client_scripts) = manifest.client_scripts {
            for script_path in client_scripts {
                let Some(script_file) = FileAccess::open(script_path.clone().into(), ModeFlags::READ) else {
                    return Err(format!("Manifest {} script {} file error", manifest_path, script_path));
                };
                let script_text: String = script_file.get_as_text().into();
                resource.scripts.insert(script_path.replace("res://", ""), script_text);
            }
        }

        if let Some(media_list) = manifest.media {
            for media_path in media_list {
                let Some(_media_file) = FileAccess::open(media_path.clone().into(), ModeFlags::READ) else {
                    return Err(format!("Manifest {} media {} file error", manifest_path, media_path));
                };
                let bytes = FileAccess::get_file_as_bytes(media_path.clone().into());
                resource.media.insert(media_path.replace("res://", ""), bytes.to_vec());
            }
        }
        result.push(resource);
    }

    return Ok(result);
}
