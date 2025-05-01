use std::f32::consts::PI;

use bevy::{core_pipeline::experimental::taa::TemporalAntiAliasPlugin, pbr::CascadeShadowConfigBuilder, prelude::*, window::PresentMode};
use network::client::NetworkPlugin;
use player_controller::controller::PlayerControllerPlugin;
use world::worlds_manager::WorldsManagerPlugin;

pub mod client_scripts;
pub mod network;
pub mod player_controller;
pub mod utils;
pub mod world;

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
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .build(),
    ));

}
