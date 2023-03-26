use godot::prelude::*;
use rhai::FnPtr;
use std::cell::RefCell;
use std::rc::Rc;

use crate::console_handler::Console;

pub struct ScriptInstanceScope {
    slug: String,
    console: Gd<Console>,
    pub callbacks: Vec<(String, String)>,
}

pub type SharedScriptInstanceScope = Rc<RefCell<ScriptInstanceScope>>;

impl ScriptInstanceScope {
    pub fn new(slug: String, main_base: &Base<Node>) -> Self {
        ScriptInstanceScope {
            slug: slug,
            console: main_base
                .get_node_as::<Console>("GUIControl/MarginContainer/ConsoleContainer"),
            callbacks: Vec::new(),
        }
    }

    pub fn add_callback(&mut self, event_slug: String, callback: &FnPtr) -> Result<(), String> {
        let fn_name = callback.fn_name().to_string();
        for c in &self.callbacks {
            if c.0 == event_slug && c.1 == fn_name {
                return Err(format!(
                    "callback for event:\"{}\" already registered with name \"{}\"",
                    event_slug, fn_name
                ));
            }
        }

        self.callbacks
            .push((event_slug, callback.fn_name().to_string()));
        return Ok(());
    }

    pub fn get_slug(&self) -> &String {
        &self.slug
    }

    pub fn console_send(&mut self, message: String) {
        self.console.bind_mut().send(message);
    }
}
