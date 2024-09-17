use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ResourceManifest {
    pub slug: String,
    pub title: Option<String>,
    pub autor: Option<String>,
    pub version: Option<String>,
    pub client_scripts: Option<Vec<String>>,
    pub media: Option<Vec<String>>,
}

/// scripts: short_path, code
pub struct ResourceInstance {
    slug: String,
    title: String,
    autor: Option<String>,
    version: Option<String>,
    scripts: HashMap<String, String>,
    media: HashMap<String, Vec<u8>>,
}

impl ResourceInstance {
    pub fn get_slug(&self) -> &String {
        &self.slug
    }
    pub fn get_title(&self) -> &String {
        &self.title
    }
    pub fn get_autor(&self) -> String {
        match &self.autor {
            Some(s) => s.clone(),
            None => "-".to_string(),
        }
    }
    pub fn get_version(&self) -> String {
        match &self.version {
            Some(s) => s.clone(),
            None => "-".to_string(),
        }
    }
    pub fn get_scripts_count(&self) -> usize {
        self.scripts.len()
    }
    pub fn get_media_count(&self) -> usize {
        self.media.len()
    }

    pub fn get_media(&self) -> &HashMap<String, Vec<u8>> {
        &self.media
    }

    pub fn get_client_scripts(&self) -> &HashMap<String, String> {
        &self.scripts
    }

    pub fn from_manifest(path: PathBuf) -> Result<Self, String> {
        let manifest_path = format!("{}/manifest.yml", path.display());
        log::info!(target: "resources", "Start loading manifest \"{}\"", manifest_path);

        let manifest_data = match std::fs::read_to_string(manifest_path.clone()) {
            Ok(d) => d,
            Err(e) => {
                return Err(format!("file error: {}", e));
            }
        };

        let manifest_result: Result<ResourceManifest, serde_yaml::Error> = serde_yaml::from_str(&manifest_data);
        let manifest = match manifest_result {
            Ok(m) => m,
            Err(e) => {
                return Err(format!("error with parse manifest yaml: {}", e));
            }
        };

        let title = match &manifest.title {
            Some(t) => t.clone(),
            None => manifest.slug.clone(),
        };
        let mut inst = ResourceInstance {
            slug: manifest.slug.clone(),
            title: title,
            autor: manifest.autor.clone(),
            version: manifest.version.clone(),
            scripts: HashMap::new(),
            media: HashMap::new(),
        };
        if let Some(client_scripts) = &manifest.client_scripts {
            for client_script in client_scripts.iter() {
                let path = format!("{}/{}", path.display(), client_script);
                let data = match std::fs::read_to_string(path) {
                    Ok(d) => d,
                    Err(e) => {
                        log::error!(target: "resources", "□ script file \"{}\" error: {:?}", client_script, e);
                        continue;
                    }
                };
                inst.scripts.insert(client_script.clone(), data);
            }
        }
        if let Some(media_list) = &manifest.media {
            for media in media_list.iter() {
                let path = format!("{}/{}", path.display(), media);
                let data = match std::fs::read(path) {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!(target: "resources", "□ media file \"{}\" error: {:?}", media, e);
                        continue;
                    }
                };
                inst.media.insert(media.clone(), data);
            }
        }
        Ok(inst)
    }
}
