use godot::builtin::Vector2;
use godot::engine::global::JoyAxis;
use godot::engine::input::MouseMode;
use godot::engine::{Input, InputEvent, InputEventJoypadMotion, InputEventMouseMotion};
use godot::prelude::Vector3;
use godot::prelude::*;

use crate::main_scene::FloatType;

use super::enums::controller_actions::ControllerActions;

const SENSITIVITY: f32 = 0.2;
const JOYAXIS_SENSITIVITY: f32 = 150.0;
const MIN_PITCH: f32 = -90.0;
const MAX_PITCH: f32 = 75.0;

#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct Controls {
    base: Base<Node>,

    pub move_rot: FloatType,
    pub horizontal_velocity: Vector3,

    cam_rot: Vector2,
    joyaxis: Vector2,
}

impl Controls {
    pub fn get_movement_vector(&mut self) -> Vector3 {
        if Input::singleton().get_mouse_mode() != MouseMode::CAPTURED {
            return Vector3::ZERO;
        }

        // Determine the movement direction based checked the input strengths of the four movement directions
        let input = Input::singleton();
        let dx = input.get_action_strength(ControllerActions::MoveRight.as_str().into())
            - input.get_action_strength(ControllerActions::MoveLeft.as_str().into());
        let dy = input.get_action_strength(ControllerActions::MoveForward.as_str().into())
            - input.get_action_strength(ControllerActions::MoveBackwards.as_str().into());

        // and set the movement direction vector to the normalized vector so the player can't unintentionally
        // move faster when moving diagonally
        return Vector3::new(dx, 0.0, -dy).normalized();
    }

    pub fn get_camera_rotation(&self) -> &Vector2 {
        return &self.cam_rot;
    }

    pub fn is_jumping(&self) -> bool {
        let input = Input::singleton();
        input.is_action_just_pressed(ControllerActions::Jump.as_str().into())
    }

    pub fn is_main_action(&self) -> bool {
        let input = Input::singleton();
        input.is_action_just_pressed(ControllerActions::ActionMain.as_str().into())
    }
}

#[godot_api]
impl INode for Controls {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            move_rot: Default::default(),
            horizontal_velocity: Default::default(),
            cam_rot: Vector2::ZERO,
            joyaxis: Vector2::ZERO,
        }
    }

    fn process(&mut self, delta: f64) {
        self.cam_rot.x += self.joyaxis.x * delta as f32 * JOYAXIS_SENSITIVITY;
        self.cam_rot.y += self.joyaxis.y * delta as f32 * JOYAXIS_SENSITIVITY;

        self.cam_rot.y = self.cam_rot.y.min(MAX_PITCH).max(MIN_PITCH);
    }

    fn input(&mut self, event: Gd<InputEvent>) {

        if let Ok(event) = event.clone().try_cast::<InputEventJoypadMotion>() {
            if event.get_axis() == JoyAxis::RIGHT_X {
                self.joyaxis.x = event.get_axis_value();
            }
            if event.get_axis() == JoyAxis::RIGHT_Y {
                self.joyaxis.y = event.get_axis_value();
            }
        }

        let input = Input::singleton();
        if input.get_mouse_mode() == MouseMode::CAPTURED {
            if let Ok(event) = event.try_cast::<InputEventMouseMotion>() {
                self.cam_rot.x += event.get_relative().x * SENSITIVITY * -1.0;
                self.cam_rot.y += event.get_relative().y * SENSITIVITY * -1.0;
            }
        }
    }
}
