use bevy::prelude::{resource_exists, IntoSystemConfig, Res};
use bevy_app::{App, CoreSchedule, IntoSystemAppConfig, Plugin, StartupSet};

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
        app.add_system(
            ConsoleHandler::run_handler
                .in_schedule(CoreSchedule::Startup)
                .in_base_set(StartupSet::PostStartup), //.run_if(resource_exists::<ServerRuntime>()),
        );
    }
}
