use godot::prelude::godot_print;
use rhai::{Dynamic, Engine};

pub fn print(msg: String) {
    godot_print!("{}", msg)
}

pub fn register_event(event_name: String, callback: Dynamic) {
    godot_print!("{}: {}", event_name, callback)
}

pub fn register_modules(rhai_engine: &mut Engine) {
    rhai_engine.register_fn("print", print);
    rhai_engine.register_fn("registerEvent", register_event);
}
