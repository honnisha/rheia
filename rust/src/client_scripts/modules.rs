use godot::prelude::godot_print;
use rhai::{Dynamic, Engine, NativeCallContext};

pub fn fn_godot_print(context: NativeCallContext, msg: String) {
    godot_print!(
        "[{}] {}",
        context.global_runtime_state().source.as_ref().unwrap(),
        msg
    );
}

pub fn register_event(context: NativeCallContext, event_name: String, callback: Dynamic) {
    godot_print!(
        "[{}] Event registered for \"{}\": {}",
        context.global_runtime_state().source.as_ref().unwrap(),
        event_name,
        callback
    );
}

pub fn register_modules(rhai_engine: &mut Engine) {
    rhai_engine.register_fn("godot_print", fn_godot_print);
    rhai_engine.register_fn("registerEvent", register_event);
}
