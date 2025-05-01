use rhai::{serde::to_dynamic, Dynamic, Engine};

use super::{events::EmptyEvent, script_instance::ScriptInstance};
use std::collections::HashMap;

pub enum MediaResource {
    Texture,
    GLB,
}

impl MediaResource {
    pub fn get_glb(&self)  {
        todo!()
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
        todo!()
    }

    pub fn add_media_from_resource(&mut self, media_slug: String, data: ()) -> Result<(), String> {
        todo!();
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

    pub fn _is_network(&self) -> bool {
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
