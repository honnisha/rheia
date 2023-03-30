use super::instance_scope::ScriptInstanceScope;
use super::scripts_manager::Manifest;
use godot::prelude::*;
use rhai::{Dynamic, Engine, ImmutableString, Scope, AST};
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

pub struct ScriptInstance {
    slug: String,
    title: String,
    autor: String,
    version: String,
    source: Option<String>,
    ast: Option<AST>,
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
            ast: None,
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
            self.ast = Some(ast);
        }
        Ok(())
    }

    pub fn run_event(&mut self, rhai_engine: &Engine, event_slug: &String, attrs: &Vec<Dynamic>) {
        let callbacks: Vec<(String, String)>;
        let slug: String;
        let si = self.scope_instance.clone();
        {
            callbacks = si.borrow().callbacks.clone();
            slug = si.borrow().get_slug().clone();
        }

        for callback in callbacks {
            if &callback.0 == event_slug {
                // Call callback
                godot_print!("CALL_FN event_slug:{} callback:{:?}", event_slug, callback);
                let callback_result = rhai_engine.call_fn::<()>(
                    &mut self.scope,
                    &self.ast.as_ref().unwrap(),
                    &callback.1,
                    attrs.clone(),
                );
                godot_print!("event_slug:{} callback:{:?} callback_result:{:?}", event_slug, callback, callback_result);
                if callback_result.is_err() {
                    let m = format!(
                        "[{}] Event {} callback \"{}\" error: {:?}",
                        slug,
                        event_slug,
                        callback.1,
                        callback_result.err()
                    );
                    let mut sc = self.scope_instance.borrow_mut();
                    sc.console_send(m);
                }
                godot_print!("event {} fired for {}", event_slug, slug)
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
