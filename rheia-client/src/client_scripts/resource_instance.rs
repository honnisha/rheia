use godot::{
    builtin::PackedByteArray,
    classes::{
        portable_compressed_texture_2d::CompressionMode, CompressedTexture2D, Image, Node3D, PackedScene,
        PortableCompressedTexture2D, Resource, Texture2D,
    },
    obj::{Gd, NewGd},
};
use rhai::{serde::to_dynamic, Dynamic, Engine};

use crate::utils::glb::glb_import;

use super::{events::EmptyEvent, script_instance::ScriptInstance};
use std::collections::HashMap;

pub enum MediaResource {
    Texture(Gd<Texture2D>),
    GLB(Gd<Node3D>),
}

impl MediaResource {
    pub fn get_glb(&self) -> Option<&Gd<Node3D>> {
        match self {
            MediaResource::GLB(g) => Some(g),
            _ => return None,
        }
    }
}

#[derive(Default)]
pub struct ResourceInstance {
    slug: String,
    scripts: Vec<ScriptInstance>,
    media: HashMap<String, MediaResource>,

    #[allow(dead_code)]
    is_network: bool,
}

impl ResourceInstance {
    pub fn iter_media(&self) -> &HashMap<String, MediaResource> {
        &self.media
    }

    pub fn get_scripts_count(&self) -> usize {
        self.scripts.len()
    }

    pub fn get_media_count(&self) -> usize {
        self.media.len()
    }

    pub fn new(slug: String, is_network: bool) -> Self {
        Self {
            slug,
            is_network,
            ..Default::default()
        }
    }

    pub fn add_script(&mut self, rhai_engine: &mut Engine, slug: String, code: String) -> Result<(), String> {
        match ScriptInstance::try_to_load(rhai_engine, slug, code) {
            Ok(i) => self.scripts.push(i),
            Err(e) => {
                return Err(format!("rhai script error:{}", e));
            }
        }
        Ok(())
    }

    pub fn add_media_from_bytes(&mut self, media_slug: String, data: Vec<u8>) -> Result<(), String> {
        let resource = if media_slug.ends_with(".png") {
            let mut pba = PackedByteArray::new();
            pba.extend(data);

            let mut image = Image::new_gd();
            image.load_png_from_buffer(&pba);

            let mut texture = PortableCompressedTexture2D::new_gd();
            image.set_name(&format!("Image \"{}\"", media_slug));
            texture.create_from_image(&image, CompressionMode::LOSSY);
            MediaResource::Texture(texture.upcast())
        } else if media_slug.ends_with(".glb") {
            let mut glb = match glb_import(data) {
                Ok(g) => g,
                Err(e) => return Err(e),
            };
            glb.set_name(&format!("GLB model \"{}\"", media_slug));
            MediaResource::GLB(glb)
        } else {
            return Err("this filetype is not supported".to_string());
        };

        self.media.insert(media_slug.clone(), resource);
        log::debug!(target:"resources", "Resource \"{}\" media \"{}\" loaded", self.slug, media_slug);
        Ok(())
    }

    pub fn add_media_from_resource(&mut self, media_slug: String, data: Gd<Resource>) -> Result<(), String> {
        let resource = if media_slug.ends_with(".png") {
            let texture = data.cast::<CompressedTexture2D>();
            MediaResource::Texture(texture.upcast())
        } else if media_slug.ends_with(".glb") {
            let glb = data.cast::<PackedScene>().instantiate_as::<Node3D>();
            MediaResource::GLB(glb)
        } else {
            return Err("this filetype is not supported".to_string());
        };
        self.media.insert(media_slug.clone(), resource);
        log::debug!(target:"resources", "Resource \"{}\" media \"{}\" loaded", self.slug, media_slug);
        Ok(())
    }

    pub fn _run_event(&mut self, rhai_engine: &mut Engine, callback_name: &String, attrs: &Vec<Dynamic>) {
        for script in self.scripts.iter_mut() {
            let option_fn = script.get_scope_instance().borrow().get_callback_fn(callback_name);

            if let Some(fn_name) = option_fn {
                let bind = EmptyEvent {};
                let _result = script._run_fn(&rhai_engine, &fn_name, attrs, &mut to_dynamic(bind).unwrap());
            }
        }
    }

    pub fn get_slug(&self) -> &String {
        &self.slug
    }

    pub fn has_media(&self, slug: &String) -> bool {
        self.media.contains_key(slug)
    }

    pub fn get_media(&self, slug: &String) -> Option<&MediaResource> {
        self.media.get(slug)
    }

    pub fn is_network(&self) -> bool {
        self.is_network
    }
}

impl AsRef<ScriptInstance> for ScriptInstance {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl AsMut<ScriptInstance> for ScriptInstance {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}
