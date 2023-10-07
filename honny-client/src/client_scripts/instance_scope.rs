use rhai::FnPtr;
use std::cell::RefCell;
use std::rc::Rc;

use crate::console::console_handler::Console;

pub struct ScriptInstanceScope {
    slug: String,

    // Callback slug, function handler name
    callbacks: Vec<(String, String)>,
}

pub type SharedScriptInstanceScope = Rc<RefCell<ScriptInstanceScope>>;

impl ScriptInstanceScope {
    pub fn get_callback_fn(&self, event_slug: &String) -> Option<String> {
        for (callback, fn_name) in self.callbacks.iter() {
            if callback == event_slug {
                return Some(fn_name.clone());
            }
        }
        None
    }

    pub fn new(slug: String) -> Self {
        ScriptInstanceScope {
            slug: slug,
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

        self.callbacks.push((event_slug, callback.fn_name().to_string()));
        return Ok(());
    }

    pub fn get_slug(&self) -> &String {
        &self.slug
    }

    pub fn console_send(&self, message: String) {
        Console::send_message(format!("[color=gray][{}][/color] {}", self.slug, message));
    }
}
