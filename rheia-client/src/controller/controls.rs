use godot::builtin::Vector2;
use godot::engine::global::JoyAxis;
use godot::engine::input::MouseMode;
use godot::engine::{Input, InputEvent, InputEventJoypadMotion, InputEventMouseMotion};
use godot::prelude::Vector3;
use godot::prelude::*;

use super::enums::controller_actions::ControllerActions;

const SENSITIVITY: f32 = 0.2;
const JOYAXIS_SENSITIVITY: f32 = 150.0;
const JOYAXIS_DEADZONE: f32 = 0.3;
const MIN_PITCH: f32 = -90.0;
const MAX_PITCH: f32 = 75.0;

#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct Controls {
    base: Base<Node>,

    // Rotation degrees
    cam_rot: Vector2,

    movement: Vector3,

    // Joyaxis velocity
    joyaxis_right: Vector2,
    joyaxis_left: Vector2,
}

impl Controls {
    pub fn get_movement_vector(&self) -> &Vector3 {
        return &self.movement;
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

fn deadzone_threshold(value: f32) -> f32 {
    if value.abs() < JOYAXIS_DEADZONE {
        return 0.0;
    }
    value
}

#[godot_api]
impl INode for Controls {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            cam_rot: Vector2::ZERO,
            joyaxis_right: Vector2::ZERO,
            movement: Default::default(),
            joyaxis_left: Default::default(),
        }
    }

    fn process(&mut self, delta: f64) {
        self.cam_rot.x += self.joyaxis_right.x * delta as f32 * JOYAXIS_SENSITIVITY;
        self.cam_rot.y += self.joyaxis_right.y * delta as f32 * JOYAXIS_SENSITIVITY;

        self.cam_rot.y = self.cam_rot.y.min(MAX_PITCH).max(MIN_PITCH);

        //self.movement = Vector3::ZERO;
        let input = Input::singleton();
        if self.joyaxis_left == Vector2::ZERO {
            if input.get_mouse_mode() == MouseMode::CAPTURED {
                self.movement.x = input.get_action_strength(ControllerActions::MoveRight.as_str().into())
                    - input.get_action_strength(ControllerActions::MoveLeft.as_str().into());
                self.movement.z = input.get_action_strength(ControllerActions::MoveForward.as_str().into())
                    - input.get_action_strength(ControllerActions::MoveBackwards.as_str().into());
                self.movement.z *= -1.0;
            }
        } else {
            self.movement = Vector3::new(self.joyaxis_left.x, 0.0, self.joyaxis_left.y)
        }

        // and set the movement direction vector to the normalized vector so the player can't unintentionally
        // move faster when moving diagonally
        if self.movement != Vector3::ZERO {
            self.movement = self.movement.normalized();
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if let Ok(event) = event.clone().try_cast::<InputEventJoypadMotion>() {
            // Right stick
            if event.get_axis() == JoyAxis::RIGHT_X {
                self.joyaxis_right.x = deadzone_threshold(event.get_axis_value());
            }
            if event.get_axis() == JoyAxis::RIGHT_Y {
                self.joyaxis_right.y = deadzone_threshold(event.get_axis_value());
            }

            // Left stick
            if event.get_axis() == JoyAxis::LEFT_X {
                self.joyaxis_left.x = deadzone_threshold(event.get_axis_value());
            }
            if event.get_axis() == JoyAxis::LEFT_Y {
                self.joyaxis_left.y = deadzone_threshold(event.get_axis_value());
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
