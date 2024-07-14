use std::f32::consts::PI;

use bevy::{color::palettes::css::SILVER, core_pipeline::experimental::taa::TemporalAntiAliasPlugin, prelude::*, window::PresentMode};
use network::client::NetworkPlugin;
use player_controller::controller::PlayerControllerPlugin;
use world::worlds_manager::WorldsManagerPlugin;

pub mod network;
pub mod player_controller;
pub mod utils;
pub mod world;

// Network Renet
pub type NetworkClientType = common::network::renet::client::RenetClientNetwork;

// Network RakNet
// pub type NetworkClientType = common::network::rak_rs::client::RakNetClientNetwork;

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

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(10.0, 10.0).subdivisions(10)),
        material: materials.add(Color::from(SILVER)),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, PI * -0.15, PI * -0.15)),
        ..default()
    });
}
