use serde::{Deserialize, Serialize};
use std::collections::hash_map::Iter;
use std::{collections::HashMap, path::PathBuf};

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

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct MediaMeta {
    path: String,
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

    pub fn has_media(&self, slug: &String) -> bool {
        self.media.contains_key(slug)
    }

    pub fn iter_scripts(&self) -> Iter<'_, String, String> {
        self.scripts.iter()
    }

    pub fn iter_media(&self) -> Iter<'_, String, Vec<u8>> {
        self.media.iter()
    }

    pub fn get_media(&self) -> &HashMap<String, Vec<u8>> {
        &self.media
    }

    pub fn get_client_scripts(&self) -> &HashMap<String, String> {
        &self.scripts
    }

    pub fn empty(slug: String) -> Self {
        Self {
            slug: slug.clone(),
            title: slug,
            autor: None,
            version: None,
            scripts: Default::default(),
            media: Default::default(),
        }
    }

    pub fn from_manifest(resource_path: PathBuf) -> Result<Self, String> {
        let mut manifest_path = resource_path.clone();
        manifest_path.push("manifest.yml");

        log::info!(target: "resources", "Start loading manifest &e\"{}\"", manifest_path.display());

        let manifest_data = match std::fs::read_to_string(manifest_path.clone()) {
            Ok(d) => d,
            Err(e) => {
                return Err(format!("file error: &c{}", e));
            }
        };

        let manifest_result: Result<ResourceManifest, serde_yaml::Error> = serde_yaml::from_str(&manifest_data);
        let manifest = match manifest_result {
            Ok(m) => m,
            Err(e) => {
                return Err(format!("error with parse manifest yaml: &c{}", e));
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
                let mut script_path = resource_path.clone();
                script_path.push(client_script);

                let data = match std::fs::read_to_string(script_path) {
                    Ok(d) => d,
                    Err(e) => {
                        log::error!(target: "resources", "□ script file &e\"{}\"&r error: &c{:?}", client_script, e);
                        continue;
                    }
                };
                inst.add_script(client_script.clone(), data);
            }
        }
        if let Some(media_list) = &manifest.media {
            for media in media_list.iter() {
                let mut media_path = resource_path.clone();
                media_path.push(media);

                let data = match std::fs::read(media_path.clone()) {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!(target: "resources", "□ media content file &e\"{}\"&r error: &c{:?}", media_path.display(), e);
                        continue;
                    }
                };
                inst.add_media(media.clone(), data);
            }
        }
        Ok(inst)
    }

    pub fn add_script(&mut self, slug: String, data: String) {
        self.scripts.insert(slug, data);
    }

    pub fn add_media(&mut self, slug: String, data: Vec<u8>) {
        self.media.insert(slug, data);
    }
}
