use godot::prelude::*;
use rhai::Engine;
use serde::{Deserialize, Serialize};
use serde_yaml::Error;
use std::env;
use std::fs;

pub struct ScriptsManager {
    rhai_engine: Engine,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Manifest {
    title: String,
    autor: String,
    version: String,
    client_scripts: Vec<String>,
}

impl ScriptsManager {
    pub fn new() -> Self {
        ScriptsManager {
            rhai_engine: Engine::new(),
        }
    }

    pub fn rescan_scripts(&self) {
        let mut path = env::current_dir().unwrap().clone();
        path.pop();
        path.push("resources");
        let path_str = path.into_os_string().into_string().unwrap();
        godot_print!("Rescan script folders inside: {}", path_str);

        match fs::read_dir(path_str) {
            Ok(paths) => {
                for path in paths {
                    let current_path = path.unwrap().path();

                    let manifest_path = format!("{}/manifest.yml", current_path.display());
                    match fs::read_to_string(manifest_path.clone()) {
                        Ok(data) => {
                            let manifest_result: Result<Manifest, Error> =
                                serde_yaml::from_str(&data);
                            match manifest_result {
                                Ok(manifest) => {
                                    self.load(manifest, data, current_path.display().to_string())
                                }
                                Err(e) => {
                                    godot_print!(
                                        "- error with parse manifest yaml {}: {}",
                                        manifest_path,
                                        e
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            godot_print!("- error with manifest file {}: {}", manifest_path, e);
                        }
                    }
                }
            }
            Err(e) => {
                godot_print!("Error: {}", e);
            }
        }
    }

    pub fn load(&self, manifest: Manifest, data: String, path: String) {
        godot_print!(
            "- loaded script \"{}\" author:\"{}\" version:{}",
            manifest.title,
            manifest.autor,
            manifest.version
        );
    }
}
