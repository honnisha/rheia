use super::scripts_manager::Manifest;
use godot::prelude::godot_print;
use rhai::{Engine, Scope};
use std::fs;

pub struct ScriptInstance {
    slug: String,
    title: String,
    autor: String,
    version: String,
    source: Option<String>,
}

impl ScriptInstance {
    pub fn from_manifest(manifest: &Manifest, source: String) -> Self {

        ScriptInstance {
            slug: manifest.slug.clone(),
            title: manifest.autor.clone(),
            autor: manifest.autor.clone(),
            version: manifest.autor.clone(),
            source: Some(source.clone()),
        }
    }

    #[allow(dead_code)]
    pub fn get_slug(&self) -> &String {
        &self.slug
    }

    #[allow(dead_code)]
    pub fn get_title(&self) -> &String {
        &self.title
    }

    #[allow(dead_code)]
    pub fn get_autor(&self) -> &String {
        &self.autor
    }

    #[allow(dead_code)]
    pub fn get_version(&self) -> &String {
        &self.version
    }

    pub fn try_to_load(
        &self,
        rhai_engine: &mut Engine,
        client_scripts: &Vec<String>,
    ) -> Result<(), String> {
        let mut scope = Scope::new();

        for client_script in client_scripts {
            let path = format!("{}/{}", self.source.as_ref().unwrap(), client_script);
            let data = match fs::read_to_string(path.clone()) {
                Ok(d) => d,
                Err(e) => {
                    return Err(format!("Resource {} rhai \"{}\" load error: {}", self.get_slug(), client_script, e).into());
                }
            };

            match rhai_engine.run_with_scope(&mut scope, &data) {
                Ok(()) => (),
                Err(e) => {
                    return Err(format!("Resource {} rhai \"{}\" syntax error: {}", self.get_slug(), client_script, e).into());
                }
            };
            godot_print!("- Resource {} rhai \"{}\" loaded: {}", self.get_slug(), client_script, path);
        }
        Ok(())
    }
}