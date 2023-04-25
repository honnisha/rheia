use std::fs;
use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ResourceManifest {
    pub slug: String,
    pub title: String,
    pub autor: String,
    pub version: String,
    pub client_scripts: Vec<String>,
}

pub struct ResourceInstance {
    slug: String,
    title: String,
    autor: String,
    version: String,
    scripts: HashMap<String, String>,
}

impl ResourceInstance {
    pub fn get_slug(&self) -> &String {
        &self.slug
    }
    pub fn get_title(&self) -> &String {
        &self.title
    }
    pub fn get_autor(&self) -> &String {
        &self.autor
    }
    pub fn get_version(&self) -> &String {
        &self.version
    }

    pub fn from_manifest(manifest: &ResourceManifest, path: PathBuf) -> Result<Self, String> {
        let mut inst = ResourceInstance {
            slug: manifest.slug.clone(),
            title: manifest.title.clone(),
            autor: manifest.autor.clone(),
            version: manifest.version.clone(),
            scripts: HashMap::new(),
        };
        for client_script in manifest.client_scripts.iter() {
            let script_path = format!("{}/{}", path.display(), client_script);

            let data = match fs::read_to_string(script_path) {
                Ok(d) => d,
                Err(e) => {
                    return Err(format!("□ script file {} error: {:?}", client_script, e).into());
                }
            };
            inst.scripts.insert(client_script.clone(), data);
        }
        Ok(inst)
    }
}
