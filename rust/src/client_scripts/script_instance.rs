use super::instance_scope::ScriptInstanceScope;
use super::scripts_manager::Manifest;
use godot::prelude::*;
use rhai::{Engine, ImmutableString, Scope, Dynamic};
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

pub struct ScriptInstance {
    slug: String,
    title: String,
    autor: String,
    version: String,
    source: Option<String>,
    scope: Scope<'static>,
    scope_instance: Rc<RefCell<ScriptInstanceScope>>,
}

impl ScriptInstance {
    pub fn from_manifest(manifest: &Manifest, source: String, main_base: &Base<Node>) -> Self {
        let shared_controller = ScriptInstanceScope::new(manifest.slug.clone(), main_base);

        let mut script_instance = ScriptInstance {
            slug: manifest.slug.clone(),
            title: manifest.autor.clone(),
            autor: manifest.autor.clone(),
            version: manifest.autor.clone(),
            source: Some(source.clone()),
            scope: Scope::new(),
            scope_instance: Rc::new(RefCell::new(shared_controller)),
        };
        script_instance
            .scope
            .push_constant("Main", script_instance.scope_instance.clone());
        script_instance
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
        &mut self,
        rhai_engine: &mut Engine,
        client_scripts: &Vec<String>,
    ) -> Result<(), String> {
        for client_script in client_scripts {
            let path = format!("{}/{}", self.source.as_ref().unwrap(), client_script);
            let data = match fs::read_to_string(path.clone()) {
                Ok(d) => d,
                Err(e) => {
                    return Err(format!(
                        "○ Script {} rhai \"{}\" load error: {}",
                        self.get_slug(),
                        client_script,
                        e
                    )
                    .into());
                }
            };

            let mut ast = match rhai_engine.compile(&data) {
                Ok(a) => a,
                Err(e) => {
                    return Err(format!(
                        "○ Script {} rhai \"{}\" syntax error: {}",
                        self.get_slug(),
                        client_script,
                        e
                    )
                    .into());
                }
            };
            ast.set_source(ImmutableString::from(&self.slug));
            match rhai_engine.run_ast_with_scope(&mut self.scope, &ast) {
                Ok(()) => (),
                Err(e) => {
                    return Err(format!(
                        "○ Script \"{}\" rhai \"{}\" syntax error: {}",
                        ast.source().unwrap(),
                        client_script,
                        e
                    )
                    .into());
                }
            };
            godot_print!(
                "● Script \"{}\" rhai \"{}\" loaded: {}",
                ast.source().unwrap(),
                client_script,
                path
            );
        }
        Ok(())
    }

    pub fn run_event(&mut self, event_slug: &String, attrs: &Vec<Dynamic>) {
    }
}
