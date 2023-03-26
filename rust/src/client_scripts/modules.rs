use godot::prelude::godot_print;
use rhai::plugin::*;
use rhai::{FnPtr, NativeCallContext};

#[export_module]
pub mod main_api {
    use crate::client_scripts::instance_scope::SharedScriptInstanceScope;

    pub type Main = SharedScriptInstanceScope;

    #[rhai_fn(pure)]
    pub fn register_event(main: &mut Main, event_slug: String, callback: FnPtr) {
        let add_result = main
            .borrow_mut()
            .add_callback(event_slug.clone(), &callback);
        if add_result.is_err() {
            console(
                main,
                format!(
                    "[{}] register_event error: {:?}",
                    main.borrow().get_slug(),
                    add_result.err()
                ),
            );
            return;
        }
        console(
            main,
            format!(
                "[{}] Event registered for \"{}\": {}",
                main.borrow().get_slug(),
                event_slug,
                callback
            ),
        );
    }

    #[rhai_fn(pure)]
    pub fn get_slug(main: &mut Main) -> String {
        main.borrow().get_slug().clone()
    }

    #[rhai_fn(pure)]
    pub fn console(main: &mut Main, message: String) {
        godot_print!("[{}] {}", main.borrow().get_slug().clone(), message,);
        main.borrow_mut().console_send(message);
    }
}
