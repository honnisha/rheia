use godot::prelude::*;

use crate::main_scene::FloatType;

use super::{controls::Controls, player_controller::CAMERA_DISTANCE};

#[derive(GodotClass)]
#[class(no_init, base=Node3D)]
pub struct CameraController {
    base: Base<Node3D>,
    camera: Gd<Camera3D>,
    controls: Gd<Controls>,
}

impl CameraController {
    pub fn create(base: Base<Node3D>, controls: Gd<Controls>) -> Self {
        Self {
            base,
            camera: Camera3D::new_alloc(),

            controls,
        }
    }

    /// Horisontal degrees
    pub fn get_yaw(&self) -> f32 {
        self.base().get_rotation_degrees().y
    }

    /// Vertical degrees
    pub fn get_pitch(&self) -> f32 {
        self.base().get_rotation_degrees().x
    }

    pub fn get_camera(&self) -> &Gd<Camera3D> {
        &self.camera
    }

    pub fn rotate(&mut self, pitch: FloatType, yaw: FloatType) {
        let mut r = self.base().get_rotation_degrees();
        r.x = pitch % 360.0;
        r.y = yaw % 360.0;
        self.base_mut().set_rotation_degrees(r);
    }
}

#[godot_api]
impl INode3D for CameraController {
    fn ready(&mut self) {
        let camera = self.camera.clone().upcast();
        self.base_mut().add_child(camera);

        let mut t = self.camera.get_transform();
        t.origin.z = CAMERA_DISTANCE;
        self.camera.set_transform(t);
    }

    fn process(&mut self, _delta: f64) {
        let cam_rot = {
            let controls = self.controls.bind();
            controls.get_camera_rotation().clone()
        };

        self.rotate(cam_rot.y, cam_rot.x);
    }
}
