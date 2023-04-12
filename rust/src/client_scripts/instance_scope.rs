use clap::Command;
use godot::prelude::*;
use rhai::serde::from_dynamic;
use rhai::{Dynamic, FnPtr};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::console::command_info::CommandInfo;
use crate::console::console_handler::Console;

pub struct ScriptInstanceScope {
    slug: String,

    // Callback slug, function handler name
    callbacks: Vec<(String, String)>,

    // Function handler name,
    commands: HashMap<String, Command>,
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

    pub fn get_command(&self, lead_command: String) -> Option<(String, Command)> {
        for (fn_name, command_info) in self.commands.iter() {
            if command_info.get_name() == lead_command {
                return Some((fn_name.clone(), command_info.clone()));
            }
        }
        None
    }

    pub fn new(slug: String) -> Self {
        ScriptInstanceScope {
            slug: slug,
            callbacks: Vec::new(),
            commands: HashMap::new(),
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

    pub fn add_command(&mut self, callback: &FnPtr, command_info: Dynamic) -> Result<(), String> {
        let info: CommandInfo = match from_dynamic(&command_info) {
            Ok(ci) => ci,
            Err(e) => {
                return Err(format!("CommandInfo for:\"{}\" error: {}", callback, e,));
            }
        };
        let fn_name = callback.fn_name().to_string();
        self.commands.insert(fn_name, info.get_command());
        return Ok(());
    }

    pub fn get_slug(&self) -> &String {
        &self.slug
    }

    pub fn console_send(&mut self, message: String) {
        Console::send_message(format!("[{}] {}", self.slug, message));
    }
}
