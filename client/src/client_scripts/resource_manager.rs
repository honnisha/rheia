use rhai::exported_module;
use rhai::serde::to_dynamic;
use rhai::Dynamic;
use rhai::Engine;
use std::collections::HashMap;

use crate::events::EmptyEvent;

use super::modules::main_api;
use super::resource_instance::ResourceInstance;

pub struct ScriptsManager {
    rhai_engine: Engine,
    resources: HashMap<String, ResourceInstance>,
}

pub const REGEX_COMMAND: &str = r####"([\d\w$&+,:;=?@#|'<>.^*()%!-]+)|"([\d\w$&+,:;=?@#|'<>.^*()%!\- ]+)""####;

impl ScriptsManager {
    pub fn new() -> Self {
        let mut engine = Engine::new();

        engine.register_global_module(exported_module!(main_api).into());

        ScriptsManager {
            rhai_engine: engine,
            resources: HashMap::new(),
        }
    }

    pub fn try_load(&mut self, slug: String, scripts: HashMap<String, String>) -> Result<(), String> {
        match ResourceInstance::try_init(&mut self.rhai_engine, slug, scripts) {
            Ok(i) => self.resources.insert(slug, i),
            Err(e) => {
                return Err(format!("resource \"{}\" error: {:?}", slug, e).into());
            }
        };
        Ok(())
    }
}
