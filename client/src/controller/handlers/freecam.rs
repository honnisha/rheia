use godot::engine::{
    global::Key, global::MouseButton, input::MouseMode, InputEvent, InputEventKey, InputEventMouseButton,
    InputEventMouseMotion,
};
use godot::prelude::utilities::deg_to_rad;
use godot::prelude::*;
use std::fmt::{self, Display, Formatter};

use crate::{console::console_handler::Console, controller::player_controller::PlayerMovement, main_scene::FloatType};

const ACCELERATION: f32 = 5.0;
const BOST_MULTIPLIER: f32 = 2.5;

const SENSITIVITY: f32 = 0.25;

#[derive(Default)]
struct FreeCameraData {
    right: FloatType,
    left: FloatType,
    forward: FloatType,
    back: FloatType,

    multiplier: bool,

    mouse_position: Vector2,
    total_pitch: f32,
}

impl Display for FreeCameraData {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "(r:{} l:{} f:{} b:{}, m:{} mp:{} tp:{})",
            self.right, self.left, self.forward, self.back, self.multiplier, self.mouse_position, self.total_pitch,
        )
    }
}

impl FreeCameraData {
    fn get_movement_vector(&mut self, delta: f64) -> Vector3 {
        let mut direction = Vector3::new(self.right - self.left, 0.0, self.back - self.forward);

        direction = direction.normalized() * ACCELERATION * delta as f32;

        // Compute modifiers' speed multiplier
        if self.multiplier {
            direction *= BOST_MULTIPLIER;
        }

        return direction;
    }

    fn get_mouselook_vector(&mut self) -> (f64, f64) {
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

pub struct FreeCameraHandler {
    data: FreeCameraData,
    cache_movement: Option<PlayerMovement>,
}

impl FreeCameraHandler {
    pub fn create() -> Self {
        Self {
            data: FreeCameraData::default(),
            cache_movement: None,
        }
    }

    pub fn input(&mut self, event: Gd<InputEvent>, _camera: &mut Camera3D) {
        if Console::is_active() {
            return;
        }

        if let Some(e) = event.share().try_cast::<InputEventMouseMotion>() {
            self.data.mouse_position = e.get_relative();
        }

        if let Some(e) = event.share().try_cast::<InputEventMouseButton>() {
            if e.get_button_index() == MouseButton::MOUSE_BUTTON_RIGHT {
                let mouse_mode = match e.is_pressed() {
                    true => MouseMode::MOUSE_MODE_CAPTURED,
                    false => MouseMode::MOUSE_MODE_VISIBLE,
                };
                Input::singleton().set_mouse_mode(mouse_mode);
            }
        }

        if let Some(e) = event.try_cast::<InputEventKey>() {
            match e.get_keycode() {
                Key::KEY_D => {
                    self.data.right = e.is_pressed() as i32 as FloatType;
                }
                Key::KEY_A => {
                    self.data.left = e.is_pressed() as i32 as FloatType;
                }
                Key::KEY_W => {
                    self.data.forward = e.is_pressed() as i32 as FloatType;
                }
                Key::KEY_S => {
                    self.data.back = e.is_pressed() as i32 as FloatType;
                }
                Key::KEY_SHIFT => {
                    self.data.multiplier = e.is_pressed();
                }
                _ => (),
            };
        }
    }

    pub fn process(&mut self, base: &mut Base<Node>, delta: f64, camera: &mut Camera3D) {
        if Console::is_active() {
            return;
        }

        if Input::singleton().get_mouse_mode() == MouseMode::MOUSE_MODE_CAPTURED {
            let (yaw, pitch) = self.data.get_mouselook_vector();
            camera.rotate_y(yaw as f32);
            camera.rotate_object_local(Vector3::new(1.0, 0.0, 0.0), pitch as f32);
        }
        camera.translate(self.data.get_movement_vector(delta));
        let new_movement =
            PlayerMovement::create(camera.get_position(), camera.get_rotation().y, camera.get_rotation().x);

        if let Some(cache_movement) = self.cache_movement {
            if new_movement == cache_movement {
                return;
            }
        }

        base.emit_signal("on_player_move".into(), &[new_movement.to_variant()]);
        self.cache_movement = Some(new_movement);
    }
}
