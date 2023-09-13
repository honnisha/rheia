use godot::engine::{
    global::Key, global::MouseButton, input::MouseMode, InputEvent, InputEventKey, InputEventMouseButton,
    InputEventMouseMotion,
};
use godot::prelude::*;

use crate::console::console_handler::Console;
use crate::main_scene::FloatType;

use super::{input_data::InputData, player_movement::PlayerMovement};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct PlayerController {
    #[base]
    pub(crate) base: Base<Node>,
    camera: Gd<Camera3D>,
    input_data: InputData,
    cache_movement: Option<PlayerMovement>,
}

impl PlayerController {
    pub fn create(base: Base<Node>, camera: &Gd<Camera3D>) -> Self {
        Self {
            base,
            camera: camera.share(),
            input_data: Default::default(),
            cache_movement: None,
        }
    }

    /// Handle network packet for changing position
    pub fn teleport(&mut self, position: Vector3, yaw: FloatType, pitch: FloatType) {
        self.camera.set_position(position);
        self.camera.rotate_y(yaw);
        self.camera.rotate_object_local(Vector3::new(1.0, 0.0, 0.0), pitch as f32);
    }

    // Get position of the controller
    pub fn get_position(&self) -> Vector3 {
        self.camera.get_position()
    }

    pub fn get_yaw(&self) -> f32 {
        self.camera.get_rotation().x
    }

    pub fn get_pitch(&self) -> f32 {
        self.camera.get_rotation().y
    }
}

#[godot_api]
impl PlayerController {
    #[signal]
    fn on_player_move();
}

#[godot_api]
impl NodeVirtual for PlayerController {
    fn init(base: Base<Node>) -> Self {
        let camera = load::<PackedScene>("res://scenes/camera_3d.tscn").instantiate_as::<Camera3D>();
        Self::create(base, &camera)
    }

    fn ready(&mut self) {}

    fn input(&mut self, event: Gd<InputEvent>) {
        if Console::is_active() {
            return;
        }

        if let Some(e) = event.share().try_cast::<InputEventMouseMotion>() {
            self.input_data.mouse_position = e.get_relative();
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
                    self.input_data.right = e.is_pressed() as i32 as FloatType;
                }
                Key::KEY_A => {
                    self.input_data.left = e.is_pressed() as i32 as FloatType;
                }
                Key::KEY_W => {
                    self.input_data.forward = e.is_pressed() as i32 as FloatType;
                }
                Key::KEY_S => {
                    self.input_data.back = e.is_pressed() as i32 as FloatType;
                }
                Key::KEY_SHIFT => {
                    self.input_data.multiplier = e.is_pressed();
                }
                _ => (),
            };
        }
    }

    #[allow(unused_variables)]
    fn process(&mut self, delta: f64) {
        let now = std::time::Instant::now();

        if Console::is_active() {
            return;
        }

        if Input::singleton().get_mouse_mode() == MouseMode::MOUSE_MODE_CAPTURED {
            let (yaw, pitch) = self.input_data.get_mouselook_vector();
            self.camera.rotate_y(yaw as f32);
            self.camera
                .rotate_object_local(Vector3::new(1.0, 0.0, 0.0), pitch as f32);
        }
        self.camera.translate(self.input_data.get_movement_vector(delta));
        let new_movement = PlayerMovement::create(
            self.camera.get_position(),
            self.camera.get_rotation().y,
            self.camera.get_rotation().x,
        );

        if let Some(cache_movement) = self.cache_movement {
            if new_movement == cache_movement {
                return;
            }
        }

        self.base.emit_signal("on_player_move".into(), &[new_movement.to_variant()]);
        self.cache_movement = Some(new_movement);

        let elapsed = now.elapsed();
        if elapsed > std::time::Duration::from_millis(3) {
            println!("PlayerController process: {:.2?}", elapsed);
        }
    }
}
