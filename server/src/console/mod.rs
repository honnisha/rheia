use bevy::prelude::{Res, IntoSystemConfig, resource_exists};
use bevy_app::{App, Plugin, IntoSystemAppConfig, CoreSchedule, StartupSet};

use crate::network::server::ServerRuntime;

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
            Self::start
                .in_schedule(CoreSchedule::Startup)
                .in_base_set(StartupSet::PostStartup)
                .run_if(resource_exists::<ServerRuntime>()),
        );
    }
}

impl ConsolePlugin {
    fn start(server_runtime: Res<ServerRuntime>) {
        ConsoleHandler::run_handler(server_runtime.as_ref())
    }
}
