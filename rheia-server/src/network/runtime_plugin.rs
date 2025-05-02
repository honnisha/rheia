use bevy::prelude::{App, EventWriter, Plugin};
use bevy_app::AppExit;
use bevy_app::{First, Startup};
use bevy_ecs::system::Res;
use lazy_static::lazy_static;
use network::messages::{NetworkMessageType, ServerMessages};
use std::sync::{Arc, RwLock};

use super::clients_container::ClientsContainer;

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

fn update_runtime(mut app_exit_events: EventWriter<AppExit>, clients: Res<ClientsContainer>) {
    if RuntimePlugin::is_stopped() {
        for (_client_id, client) in clients.iter() {
            let msg = ServerMessages::Disconnect {
                message: Some("Server shutting down".to_string()),
            };
            client.send_message(NetworkMessageType::ReliableUnordered, &msg);
        }
        app_exit_events.write(AppExit::Success);
    }
}
