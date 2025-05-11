use bevy::prelude::{App, EventWriter, Plugin};
use bevy_app::AppExit;
use bevy_app::{First, Startup};
use bevy_ecs::system::{Res, ResMut};
use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};

use crate::console::console_handler::ConsoleHandler;
use crate::worlds::worlds_manager::WorldsManager;

use super::clients_container::ClientsContainer;

lazy_static! {
    static ref SERVER_STATE: Arc<RwLock<ServerState>> = Arc::new(RwLock::new(ServerState::STARTED));
}

#[derive(PartialEq)]
enum ServerState {
    STARTED,
    ACTIVE,
    STOPPING,
    STOPPED,
}

#[derive(Default)]
pub struct RuntimePlugin;

impl RuntimePlugin {
    pub fn is_active() -> bool {
        let state = SERVER_STATE.read().unwrap();
        match &*state {
            ServerState::ACTIVE => true,
            _ => false,
        }
    }

    pub fn is_stopped() -> bool {
        let state = SERVER_STATE.read().unwrap();
        *state == ServerState::STOPPED || *state == ServerState::STOPPING
    }

    pub fn activate() {
        let mut state = SERVER_STATE.write().unwrap();
        if *state == ServerState::STARTED {
            *state = ServerState::ACTIVE;
        }
    }

    pub fn stop() {
        let mut state = SERVER_STATE.write().unwrap();
        *state = ServerState::STOPPING;
    }

    pub(crate) fn is_stopping() -> bool {
        let state = SERVER_STATE.write().unwrap();
        *state == ServerState::STOPPING
    }

    pub(crate) fn set_stoped() {
        let mut state = SERVER_STATE.write().unwrap();
        *state = ServerState::STOPPING;
    }
}

impl Plugin for RuntimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, activate_runtime);
        app.add_systems(First, update_runtime);
    }
}

fn activate_runtime() {
    RuntimePlugin::activate();
}

fn update_runtime(
    mut app_exit_events: EventWriter<AppExit>,
    mut clients: ResMut<ClientsContainer>,
    mut console_handler: ResMut<ConsoleHandler>,
    worlds_manager: Res<WorldsManager>,
) {
    if RuntimePlugin::is_stopping() {
        log::info!(target: "main", "Server shutdown...");
        clients.disconnect_all(Some("Server shutting down".to_string()));
        worlds_manager.save_all().unwrap();
        console_handler.handle_stop_server();
        app_exit_events.write(AppExit::Success);
        RuntimePlugin::set_stoped();
    }
}
