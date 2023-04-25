use clap::error::ErrorKind;
use regex::Regex;
use rhai::exported_module;
use rhai::serde::to_dynamic;
use rhai::Dynamic;
use rhai::Engine;
use serde::{Deserialize, Serialize};
use serde_yaml::Error;
use std::collections::HashMap;
use std::env;
use std::fs;

use crate::console::console_handler::Console;
use crate::events::EmptyEvent;

use super::modules::main_api;
use super::script_instance::ScriptInstance;

pub struct ScriptsManager {
    rhai_engine: Engine,
    scripts: HashMap<String, ScriptInstance>,
}

pub const REGEX_COMMAND: &str = r####"([\d\w$&+,:;=?@#|'<>.^*()%!-]+)|"([\d\w$&+,:;=?@#|'<>.^*()%!\- ]+)""####;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Manifest {
    pub slug: String,
    pub title: String,
    pub autor: String,
    pub version: String,
    pub client_scripts: Vec<String>,
}

impl ScriptsManager {
    pub fn new() -> Self {
        let mut engine = Engine::new();

        engine.register_global_module(exported_module!(main_api).into());

        ScriptsManager {
            rhai_engine: engine,
            scripts: HashMap::new(),
        }
    }

    #[allow(unused_must_use)]
    pub fn run_event(&mut self, event_slug: String, attrs: Vec<Dynamic>) {
        for (_, script) in self.scripts.iter_mut() {
            let option_fn = script.get_scope_instance().borrow().get_callback_fn(&event_slug);

            if let Some(fn_name) = option_fn {
                let bind = EmptyEvent {};
                script.run_fn(&self.rhai_engine, &fn_name, &attrs, &mut to_dynamic(bind).unwrap());
            }
        }
    }
}
