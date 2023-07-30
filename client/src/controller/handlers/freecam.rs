use godot::{
    engine::{global::Key, InputEvent, InputEventKey},
    prelude::{Camera3D, Gd, Vector3, Share},
};

use crate::main_scene::FloatType;

#[derive(Default)]
struct FreeCameraKeys {
    right: FloatType,
    left: FloatType,
    forward: FloatType,
    back: FloatType,
}

impl FreeCameraKeys {
    fn as_vector(&self) -> Vector3 {
        Vector3::new(self.right - self.left, 0.0, self.forward - self.back)
    }
}

pub struct FreeCameraHandler {
    keys: FreeCameraKeys,
}

impl FreeCameraHandler {
    pub fn create() -> Self {
        Self {
            keys: FreeCameraKeys::default(),
        }
    }

    pub fn input(&mut self, event: Gd<InputEvent>, camera: &mut Camera3D) {
        //if let Some(e) = event.try_cast::<InputEventMouseButton>() {
        //    if e.get_button_index() == MouseButton::MOUSE_BUTTON_RIGHT {
        //        let mouse_mode = match e.is_pressed() {
        //            true => MouseMode::MOUSE_MODE_CAPTURED,
        //            false => MouseMode::MOUSE_MODE_VISIBLE,
        //        };
        //        Input::singleton().set_mouse_mode(mouse_mode);
        //    }
        //}

        if let Some(e) = event.share().try_cast::<InputEventKey>() {
            match e.get_keycode() {
                Key::KEY_D => {
                    self.keys.right = e.is_pressed() as i32 as FloatType;
                }
                Key::KEY_A => {
                    self.keys.left = e.is_pressed() as i32 as FloatType;
                }
                Key::KEY_W => {
                    self.keys.forward = e.is_pressed() as i32 as FloatType;
                }
                Key::KEY_W => {
                    self.keys.forward = e.is_pressed() as i32 as FloatType;
                }
                _ => (),
            };
        }
    }
}
