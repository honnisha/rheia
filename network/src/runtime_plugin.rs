use std::sync::{Arc, RwLock};

use bevy::{prelude::{Plugin, EventWriter, App, IntoSystemConfig, CoreSet}, app::AppExit};
use lazy_static::lazy_static;


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
    pub fn is_active() -> bool {
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
        app.add_startup_system(activate_runtime.in_base_set(CoreSet::Last));
        app.add_system(update_runtime.in_base_set(CoreSet::First));
    }
}

fn activate_runtime() {
    RuntimePlugin::activate();
}

fn update_runtime(mut exit: EventWriter<AppExit>) {
    if RuntimePlugin::is_stopped() {
        exit.send(AppExit)
    }
}
