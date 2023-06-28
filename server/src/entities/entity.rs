use bevy_ecs::prelude::Component;
use uuid::Uuid;

pub struct Entity(Uuid);

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}
