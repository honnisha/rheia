use godot::prelude::*;

use crate::main_scene::FloatType;

use super::controls::Controls;

const DISTANCE: f32 = 5.0;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct CameraController {
    pub(crate) base: Base<Node3D>,

    gimbal_h: Gd<Node3D>,
    gimbal_v: Gd<Node3D>,
    camera: Gd<Camera3D>,

    controls: Gd<Controls>,
}

impl CameraController {
    pub fn create(base: Base<Node3D>, controls: Gd<Controls>) -> Self {
        Self {
            base,
            gimbal_h: Node3D::new_alloc(),
            gimbal_v: Node3D::new_alloc(),
            camera: Camera3D::new_alloc(),

            controls,
        }
    }

    pub fn get_camera(&self) -> &Gd<Camera3D> {
        &self.camera
    }

    pub fn rotate(&mut self, yaw: FloatType, pitch: FloatType) {}
}

#[godot_api]
impl INode3D for CameraController {
    fn init(base: Base<Node3D>) -> Self {
        Self::create(base, Controls::new_alloc())
    }

    fn ready(&mut self) {
        let gimbal_h = self.gimbal_h.clone().upcast();
        self.base_mut().add_child(gimbal_h);

        let gimbal_v = self.gimbal_v.clone().upcast();
        self.gimbal_h.add_child(gimbal_v);

        let camera = self.camera.clone().upcast();
        self.gimbal_v.add_child(camera);

        let mut t = self.camera.get_transform();
        t.origin.z = DISTANCE;
        self.camera.set_transform(t);
    }

    fn process(&mut self, _delta: f64) {
        let controls = self.controls.bind();
        let cam_rot = controls.get_camera_rotation();

        // Prevents looking up/down too far
        let mut r = self.gimbal_v.get_rotation_degrees();
        r.x = cam_rot.y;
        self.gimbal_v.set_rotation_degrees(r);

        let mut r = self.gimbal_h.get_rotation_degrees();
        r.y = cam_rot.x;
        self.gimbal_h.set_rotation_degrees(r);
    }
}
