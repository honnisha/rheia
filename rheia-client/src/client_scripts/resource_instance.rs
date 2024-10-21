use rhai::{serde::to_dynamic, Dynamic, Engine};

use super::{events::EmptyEvent, script_instance::ScriptInstance};
use std::collections::HashMap;

#[derive(Default)]
pub struct ResourceInstance {
    slug: String,
    scripts: Vec<ScriptInstance>,
    media: HashMap<String, Vec<u8>>,
    is_network: bool,
}

impl ResourceInstance {
    pub fn try_init(
        rhai_engine: &mut Engine,
        slug: &String,
        scripts: HashMap<String, String>,
        is_network: bool,
    ) -> Result<Self, String> {
        let mut resource_instance = ResourceInstance {
            slug: slug.clone(),
            scripts: Default::default(),
            media: Default::default(),
            is_network,
        };

        for (source_file, code) in scripts {
            match ScriptInstance::try_to_load(rhai_engine, slug, source_file, code.clone()) {
                Ok(i) => resource_instance.scripts.push(i),
                Err(e) => {
                    println!("code: {}", code);
                    return Err(format!("slug \"{}\" {}", slug, e).into());
                }
            }
        }

        Ok(resource_instance)
    }

    pub fn _run_event(&mut self, rhai_engine: &mut Engine, callback_name: &String, attrs: &Vec<Dynamic>) {
        for script in self.scripts.iter_mut() {
            let option_fn = script.get_scope_instance().borrow().get_callback_fn(callback_name);

            if let Some(fn_name) = option_fn {
                let bind = EmptyEvent {};
                let _result = script.run_fn(&rhai_engine, &fn_name, attrs, &mut to_dynamic(bind).unwrap());
            }
        }
    }

    pub fn add_media(&mut self, media_slug: String, data: Vec<u8>) {
        self.media.insert(media_slug.clone(), data);
        log::trace!(target:"resources", "Resource \"{}\" media \"{}\" loaded", self.slug, media_slug);
    }

    pub fn get_media_count(&self) -> usize {
        self.media.len()
    }

    pub fn has_media(&self, slug: &String) -> bool {
        self.media.contains_key(slug)
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
