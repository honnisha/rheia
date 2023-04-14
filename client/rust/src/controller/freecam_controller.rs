use godot::prelude::Camera3D;


pub struct FreeCameraController {
}

impl FreeCameraController {
    fn init() -> Self {
        FreeCameraController {}
    }

    pub fn handle_process(_camera: &mut Camera3D, _delta: f64) {
    }
}
