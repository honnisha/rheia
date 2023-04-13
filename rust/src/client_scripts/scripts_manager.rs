use clap::error::ErrorKind;
use godot::prelude::*;
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

    pub fn rescan_scripts(&mut self) {
        let mut path = env::current_dir().unwrap().clone();
        path.pop();
        path.push("resources");
        let path_str = path.into_os_string().into_string().unwrap();
        Console::send_message(format!("▼ Rescan resources folders inside: {}", path_str));

        let paths = match fs::read_dir(path_str) {
            Ok(p) => p,
            Err(e) => {
                Console::send_message(format!("Error: {}", e));
                return ();
            }
        };

        for path in paths {
            let current_path = path.unwrap().path();

            let manifest_path = format!("{}/manifest.yml", current_path.display());

            let data = match fs::read_to_string(manifest_path.clone()) {
                Ok(d) => d,
                Err(e) => {
                    Console::send_message(format!("□ error with manifest file {}: {}", manifest_path, e));
                    continue;
                }
            };

            let manifest_result: Result<Manifest, Error> = serde_yaml::from_str(&data);
            let manifest = match manifest_result {
                Ok(m) => m,
                Err(e) => {
                    Console::send_message(format!("□ error with parse manifest yaml {}: {}", manifest_path, e));
                    continue;
                }
            };
            self.load_manifest(manifest, current_path.display().to_string());
        }
    }

    pub fn load_manifest(&mut self, manifest: Manifest, path: String) {
        let mut script_instance = ScriptInstance::from_manifest(&manifest, path);
        match script_instance.try_to_load(&mut self.rhai_engine, &manifest.client_scripts) {
            Ok(()) => (),
            Err(e) => {
                Console::send_message(format!("□ Error with manifest: {}", e));
                return ();
            }
        };

        self.scripts.insert(manifest.slug, script_instance);
        Console::send_message(format!(
            "■ loaded resource \"{}\" author:\"{}\" version:{}",
            manifest.title,
            manifest.autor,
            manifest.version
        ));
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

    #[allow(unused_must_use)]
    pub fn run_command(&mut self, command: String) -> bool {
        let re = Regex::new(REGEX_COMMAND).unwrap();
        let command_sequence: Vec<String> = re.find_iter(&command).map(|e| e.as_str().to_string()).collect();
        if command_sequence.len() == 0 {
            return false;
        }
        let lead_command = command_sequence[0].clone();

        let attrs = vec![Dynamic::from(command_sequence.clone())];

        for (_, script) in self.scripts.iter_mut() {

            let option_fn = script.get_scope_instance().borrow().get_command(lead_command.to_string());
            if let Some((fn_name, mut command)) = option_fn {
                match command.clone().try_get_matches_from(&command_sequence) {
                    Ok(_a) => {
                    },
                    Err(e) => {
                        match e.kind() {
                            ErrorKind::DisplayHelp | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand => {
                                let mut buf = Vec::new();
                                command.write_help(&mut buf).unwrap();
                                Console::send_message(String::from_utf8(buf).unwrap());
                            },
                            _ => {
                                Console::send_message(format!("[color=#DE4747]{}[/color]", e.render().to_string()));
                                println!("Error command checker: {:?}", e);
                            },
                        };
                        return true;
                    }
                }

                let bind = EmptyEvent {};
                script.run_fn(&self.rhai_engine, &fn_name, &attrs, &mut to_dynamic(bind).unwrap());
                return true;
            }
        }
        false
    }
}
