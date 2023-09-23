use godot::prelude::{utilities::deg_to_rad, Input, Vector2, Vector3};
use std::fmt::{self, Display, Formatter};

use crate::main_scene::FloatType;

const ACCELERATION: f32 = 5.0;
const BOST_MULTIPLIER: f32 = 2.5;

const SENSITIVITY: f32 = 0.25;

#[derive(Default)]
pub(crate) struct InputData {
    pub(crate) right: FloatType,
    pub(crate) left: FloatType,
    pub(crate) forward: FloatType,
    pub(crate) back: FloatType,

    pub(crate) space: bool,
    pub(crate) multiplier: bool,

    pub(crate) mouse_position: Vector2,
    pub(crate) total_pitch: f32,
}

impl Display for InputData {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "(r:{} l:{} f:{} b:{}, m:{} mp:{} tp:{})",
            self.right, self.left, self.forward, self.back, self.multiplier, self.mouse_position, self.total_pitch,
        )
    }
}

impl InputData {
    pub fn get_movement_vector(&mut self, delta: f64) -> Vector3 {
        let mut direction = Vector3::new(self.right - self.left, 0.0, self.back - self.forward);

        direction = direction.normalized() * ACCELERATION * delta as f32;

        // Compute modifiers' speed multiplier
        if self.multiplier {
            direction *= BOST_MULTIPLIER;
        }

        return direction;
    }

    pub fn get_mouselook_vector(&mut self) -> (f64, f64) {
        self.mouse_position *= SENSITIVITY;
        let yaw = self.mouse_position.x;
        let mut pitch = self.mouse_position.y;
        self.mouse_position = Vector2::new(0.0, 0.0);

        // Prevents looking up/down too far
        pitch = pitch.max(-90.0 - self.total_pitch).min(90.0 - self.total_pitch);
        self.total_pitch += pitch;

        return (deg_to_rad(-yaw as f64), deg_to_rad(-pitch as f64));
    }
}
