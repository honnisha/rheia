use bevy_app::{App, Plugin};

use self::console_handler::ConsoleHandler;

pub mod console_handler;
pub mod console_sender;

pub struct ConsolePlugin;

impl Default for ConsolePlugin {
    fn default() -> Self {
        Self {}
    }
}

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ConsoleHandler::new());
        app.add_startup_system(ConsoleHandler::run_handler);
    }
}
