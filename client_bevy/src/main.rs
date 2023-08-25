use bevy::prelude::*;
use network::client::NetworkPlugin;
use world::worlds_manager::WorldsManagerPlugin;

pub mod network;
pub mod world;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WorldsManagerPlugin::default(), NetworkPlugin::default()))
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // plane
    //commands.spawn(PbrBundle {
    //    mesh: meshes.add(shape::Plane::from_size(5.0).into()),
    //    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //    ..default()
    //});

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 60.0, 60.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
