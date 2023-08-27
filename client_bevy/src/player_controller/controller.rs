use bevy::prelude::*;
use bevy_app::App;

use super::freecam_handler::{spawn_camera, pan_orbit_camera};


#[derive(Default)]
pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, pan_orbit_camera);
        app.add_systems(Startup, spawn_camera);
    }
}
