use bevy::prelude::{Event, Events, ResMut};
use bevy_app::AppExit;
use renet::transport::NetcodeTransportError;
use log::error;

#[derive(Event)]
pub struct NetcodeErrorEvent {
    error: NetcodeTransportError,
}

impl NetcodeErrorEvent {
    pub fn new(error: NetcodeTransportError) -> Self {
        Self { error }
    }
}

pub fn netcode_error_handler(
    mut netcode_error_event: ResMut<Events<NetcodeErrorEvent>>,
    mut app_exit_events: ResMut<Events<AppExit>>,
) {
    for event in netcode_error_event.drain() {
        error!("Netcode error: {}", event.error);
        app_exit_events.send(bevy::app::AppExit);
        return;
    }
}
