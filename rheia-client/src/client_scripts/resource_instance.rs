use rhai::{serde::to_dynamic, Dynamic, Engine};

use super::{events::EmptyEvent, script_instance::ScriptInstance};
use std::collections::HashMap;

#[derive(Default)]
pub struct ResourceInstance {
    scripts: Vec<ScriptInstance>,
    media: HashMap<String, Vec<u8>>,
}

impl ResourceInstance {
    pub fn try_init(rhai_engine: &mut Engine, slug: &String, scripts: HashMap<String, String>) -> Result<Self, String> {
        let mut resource_instance = ResourceInstance::default();

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

    pub fn run_event(&mut self, rhai_engine: &mut Engine, callback_name: &String, attrs: &Vec<Dynamic>) {
        for script in self.scripts.iter_mut() {
            let option_fn = script.get_scope_instance().borrow().get_callback_fn(callback_name);

            if let Some(fn_name) = option_fn {
                let bind = EmptyEvent {};
                let _result = script.run_fn(&rhai_engine, &fn_name, attrs, &mut to_dynamic(bind).unwrap());
            }
        }
    }

    pub fn add_media(&mut self, media_slug: String, data: Vec<u8>) {
        self.media.insert(media_slug, data);
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
