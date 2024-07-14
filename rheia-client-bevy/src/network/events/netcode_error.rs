use bevy::prelude::{Event, Events, ResMut};
use bevy_app::AppExit;

#[derive(Event)]
pub struct NetcodeErrorEvent {
    error: String,
}

impl NetcodeErrorEvent {
    pub fn new(error: String) -> Self {
        Self { error }
    }
}

pub fn netcode_error_handler(
    mut netcode_error_event: ResMut<Events<NetcodeErrorEvent>>,
    mut app_exit_events: ResMut<Events<AppExit>>,
) {
    for event in netcode_error_event.drain() {
        log::error!(target: "network", "Netcode error: {}", event.error);
        app_exit_events.send(AppExit::Success);
        return;
    }
}
