use std::f32::consts::PI;

use bevy::{core_pipeline::experimental::taa::TemporalAntiAliasPlugin, prelude::*, window::PresentMode};
use network::client::NetworkPlugin;
use player_controller::controller::PlayerControllerPlugin;
use world::worlds_manager::WorldsManagerPlugin;

pub mod network;
pub mod player_controller;
pub mod utils;
pub mod world;
pub mod client_scripts;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }),
        WorldsManagerPlugin::default(),
        NetworkPlugin::default(),
        PlayerControllerPlugin::default(),
        TemporalAntiAliasPlugin,
        // FrameTimeDiagnosticsPlugin,
        // LogDiagnosticsPlugin::default(),
    ));
    app.add_systems(Startup, setup);
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, PI * -0.15, PI * -0.15)),
        ..default()
    });
}
