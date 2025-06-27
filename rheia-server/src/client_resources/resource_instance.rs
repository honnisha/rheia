use common::blocks::block_type::{BlockContent, BlockType, BlockTypeManifest};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Iter;
use std::{collections::HashMap, path::PathBuf};

const ALLOWED_FILES_EXT: &'static [&'static str] = &[".png", ".glb"];

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ResourceManifest {
    pub slug: String,
    pub title: Option<String>,
    pub autor: Option<String>,
    pub version: Option<String>,
    pub client_scripts: Option<Vec<String>>,
    pub media: Option<Vec<String>>,

    pub blocks: Option<Vec<BlockTypeManifest>>,
}

/// scripts: short_path, code
pub struct ResourceInstance {
    slug: String,
    title: String,
    autor: Option<String>,
    version: Option<String>,
    scripts: HashMap<String, String>,
    pub(crate) media: HashMap<String, Vec<u8>>,

    blocks: Vec<BlockType>,
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

    #[allow(dead_code)]
    pub fn empty(slug: String) -> Self {
        Self {
            slug: slug.clone(),
            title: slug,
            autor: None,
            version: None,
            scripts: Default::default(),
            media: Default::default(),
            blocks: Default::default(),
        }
    }

    pub fn from_manifest(resource_path: PathBuf) -> Result<Self, String> {
        let mut manifest_path = resource_path.clone();
        manifest_path.push("manifest.yml");

        log::debug!(target: "resources", "Start loading &e\"{}\"", manifest_path.display());

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
            blocks: Default::default(),
        };

        let manifest_blocks = match manifest.blocks {
            Some(b) => b,
            None => Default::default(),
        };
        for block in manifest_blocks.iter() {
            let category = match block.category.clone() {
                Some(c) => c,
                None => inst.slug.clone(),
            };
            let mut b = BlockType::new(block.block_content.clone()).category(category);
            if let Some(slug) = block.slug.as_ref() {
                b = b.set_slug(slug.clone());
            }
            b = b.visibility(block.voxel_visibility);
            inst.blocks.push(b);
        }

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
                if !ResourceInstance::is_media_allowed(&media) {
                    return Err(format!("file extension is not supported &c{}", media));
                }

                let mut media_path = resource_path.clone();
                media_path.push(media);

                let data = match std::fs::read(media_path.clone()) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "media content file &e\"{}\"&r error: &c{:?}",
                            media_path.display(),
                            e
                        ));
                    }
                };
                inst.add_media(media.clone(), data);
            }
        }

        Ok(inst)
    }

    pub(crate) fn get_blocks(&self) -> Vec<BlockType> {
        let mut blocks = self.blocks.clone();

        for block_type in blocks.iter_mut() {
            match block_type.get_block_content_mut() {
                BlockContent::Texture {
                    texture,
                    side_texture,
                    side_overlay,
                    bottom_texture,
                    ..
                } => {
                    *texture = self.local_to_global_path(&texture);
                    if let Some(texture) = side_texture {
                        *texture = self.local_to_global_path(texture);
                    }
                    if let Some(texture) = side_overlay {
                        *texture = self.local_to_global_path(texture);
                    }
                    if let Some(texture) = bottom_texture {
                        *texture = self.local_to_global_path(texture);
                    }
                }
                BlockContent::ModelCube { model, .. } => {
                    *model = self.local_to_global_path(model);
                }
            }
        }
        blocks
    }

    pub fn local_to_global_path(&self, path: &String) -> String {
        format!("{}://{}", self.get_slug(), path)
    }

    fn is_media_allowed(media: &str) -> bool {
        for ext in ALLOWED_FILES_EXT.iter() {
            if media.ends_with(ext) {
                return true;
            }
        }
        return false;
    }

    pub fn add_script(&mut self, slug: String, data: String) {
        self.scripts.insert(slug, data);
    }

    pub fn add_media(&mut self, slug: String, data: Vec<u8>) {
        self.media.insert(slug, data);
    }
}
