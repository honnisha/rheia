use godot::prelude::*;
use rhai::Engine;
use serde::{Deserialize, Serialize};
use serde_yaml::Error;
use std::collections::HashMap;
use std::env;
use std::fs;

use super::modules::register_modules;
use super::script_instance::ScriptInstance;

pub struct ScriptsManager {
    rhai_engine: Engine,
    scripts: HashMap<String, ScriptInstance>,
}

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
        register_modules(&mut engine);

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
        godot_print!("▼ Rescan resources folders inside: {}", path_str);

        let paths = match fs::read_dir(path_str) {
            Ok(p) => p,
            Err(e) => {
                godot_print!("Error: {}", e);
                return ();
            }
        };

        for path in paths {
            let current_path = path.unwrap().path();

            let manifest_path = format!("{}/manifest.yml", current_path.display());

            let data = match fs::read_to_string(manifest_path.clone()) {
                Ok(d) => d,
                Err(e) => {
                    godot_print!("□ error with manifest file {}: {}", manifest_path, e);
                    continue;
                }
            };

            let manifest_result: Result<Manifest, Error> = serde_yaml::from_str(&data);
            let manifest = match manifest_result {
                Ok(m) => m,
                Err(e) => {
                    godot_print!("□ error with parse manifest yaml {}: {}", manifest_path, e);
                    continue;
                }
            };
            self.load_manifest(manifest, current_path.display().to_string())
        }
    }

    pub fn load_manifest(&mut self, manifest: Manifest, path: String) {
        let script_instance = ScriptInstance::from_manifest(&manifest, path);
        match script_instance.try_to_load(&mut self.rhai_engine, &manifest.client_scripts) {
            Ok(()) => (),
            Err(e) => {
                godot_print!("□ Error with manifest: {}", e);
                return ();
            }
        };

        self.scripts.insert(manifest.slug, script_instance);
        godot_print!(
            "■ loaded resource \"{}\" author:\"{}\" version:{}",
            manifest.title,
            manifest.autor,
            manifest.version
        );
    }
}
