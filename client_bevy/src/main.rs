use std::f32::consts::PI;

use bevy::{
    core_pipeline::experimental::taa::TemporalAntiAliasPlugin,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use network::client::NetworkPlugin;
use player_controller::controller::PlayerControllerPlugin;
use world::worlds_manager::WorldsManagerPlugin;

pub mod network;
pub mod player_controller;
pub mod world;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        WorldsManagerPlugin::default(),
        NetworkPlugin::default(),
        PlayerControllerPlugin::default(),
        TemporalAntiAliasPlugin,
        FrameTimeDiagnosticsPlugin,
        LogDiagnosticsPlugin::default(),
    ));
    app.add_systems(Startup, setup);
    app.run();
}

fn setup(mut commands: Commands) {
    // camera
    //commands.spawn(Camera3dBundle {
    //    transform: Transform::from_xyz(0.0, 60.0, 60.0).looking_at(Vec3::ZERO, Vec3::Y),
    //    ..default()
    //});
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, PI * -0.15, PI * -0.15)),
        ..default()
    });
}
