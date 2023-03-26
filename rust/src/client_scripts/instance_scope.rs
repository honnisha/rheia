use godot::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::console_handler::Console;

pub struct ScriptInstanceScope {
    slug: String,
    console: Gd<Console>,
}

pub type SharedScriptInstanceScope = Rc<RefCell<ScriptInstanceScope>>;

impl ScriptInstanceScope {
    pub fn new(slug: String, main_base: &Base<Node>) -> Self {
        ScriptInstanceScope {
            slug: slug,
            console: main_base.get_node_as::<Console>("GUIControl/MarginContainer/ConsoleContainer"),
        }
    }

    pub fn get_slug(&self) -> &String {
        &self.slug
    }
}
