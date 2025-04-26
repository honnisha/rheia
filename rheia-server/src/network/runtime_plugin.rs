use bevy::prelude::{App, EventWriter, Plugin};
use bevy_app::AppExit;
use bevy_app::{First, Startup};
use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};

lazy_static! {
    static ref SERVER_STATE: Arc<RwLock<ServerState>> = Arc::new(RwLock::new(ServerState::STARTED));
}

#[derive(PartialEq)]
enum ServerState {
    STARTED,
    ACTIVE,
    STOPPED,
}

#[derive(Default)]
pub struct RuntimePlugin;

impl RuntimePlugin {
    pub fn _is_active() -> bool {
        let state = SERVER_STATE.read().unwrap();
        match &*state {
            ServerState::ACTIVE => true,
            _ => false,
        }
    }

    pub fn is_stopped() -> bool {
        let state = SERVER_STATE.read().unwrap();
        match &*state {
            ServerState::STOPPED => true,
            _ => false,
        }
    }

    pub fn activate() {
        let mut state = SERVER_STATE.write().unwrap();
        *state = ServerState::ACTIVE;
    }

    pub fn stop() {
        let mut state = SERVER_STATE.write().unwrap();
        *state = ServerState::STOPPED;
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

fn update_runtime(mut app_exit_events: EventWriter<AppExit>) {
    if RuntimePlugin::is_stopped() {
        app_exit_events.write(AppExit::Success);
    }
}
