use godot::builtin::Vector2;
use godot::engine::input::MouseMode;
use godot::engine::{Input, InputEvent, InputEventMouseMotion};
use godot::prelude::Vector3;
use godot::prelude::*;
use utilities::clamp;

use crate::main_scene::FloatType;

use super::enums::controller_actions::ControllerActions;

const SENSITIVITY: f32 = 0.1;
const MIN_PITCH: f64 = -90.0;
const MAX_PITCH: f64 = 75.0;

#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct Controls {
    base: Base<Node>,

    pub move_rot: FloatType,
    pub horizontal_velocity: Vector3,

    mouse_position: Vector2,
    total_pitch: f32,

    cam_rot: Vector2,
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
}

#[godot_api]
impl INode for Controls {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            move_rot: Default::default(),
            horizontal_velocity: Default::default(),
            mouse_position: Default::default(),
            total_pitch: Default::default(),
            cam_rot: Vector2::ZERO,
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        self.cam_rot = Vector2::ZERO;

        if Input::singleton().get_mouse_mode() != MouseMode::CAPTURED {
            return;
        }

        // if the mouse cursor is being captured, update the camera rotation using the relative movement
        // and the sensitivity we defined earlier. also clamp the vertical camera rotation to the pitch
        // range we defined earlier so we don't end up in weird look angles
        if let Ok(event) = event.try_cast::<InputEventMouseMotion>() {
            self.cam_rot.x -= event.get_relative().x * SENSITIVITY;
            self.cam_rot.y -= event.get_relative().y * SENSITIVITY;
            self.cam_rot.y = clamp(self.cam_rot.y.to_variant(), MIN_PITCH, MAX_PITCH);
        }
    }
}
