use rhai::{serde::to_dynamic, Dynamic, Engine};

use crate::events::EmptyEvent;

use super::script_instance::ScriptInstance;
use std::collections::HashMap;

pub struct ResourceInstance {
    slug: String,
    scripts: Vec<ScriptInstance>,
}

impl ResourceInstance {
    pub fn try_init(rhai_engine: &mut Engine, slug: String, scripts: HashMap<String, String>) -> Result<Self, String> {
        let resource_instance = ResourceInstance {
            slug: slug,
            scripts: Vec::new(),
        };

        for (source_file, code) in scripts {
            match ScriptInstance::try_to_load(rhai_engine, slug, source_file, code) {
                Ok(i) => resource_instance.scripts.push(i),
                Err(e) => return Err(format!("script error: {:?}", e).into()),
            }
        }

        Ok(resource_instance)
    }

    pub fn get_slug(&self) -> &String {
        &self.slug
    }

    pub fn run_event(&mut self, rhai_engine: &mut Engine, event_slug: String, attrs: Vec<Dynamic>) {
        for script in self.scripts.iter_mut() {
            let option_fn = script.get_scope_instance().borrow().get_callback_fn(&event_slug);

            if let Some(fn_name) = option_fn {
                let bind = EmptyEvent {};
                script.run_fn(&rhai_engine, &fn_name, &attrs, &mut to_dynamic(bind).unwrap());
            }
        }
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
