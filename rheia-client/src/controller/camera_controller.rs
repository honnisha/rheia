use godot::prelude::*;
use utilities::{deg_to_rad, lerpf};

use super::controls::Controls;

const ROTATE_SPEED: f64 = 10.0;
const ZOOM_SPEED: f64 = 10.0;
const DISTANCE: f64 = 2.0;

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
}

#[godot_api]
impl INode3D for CameraController {
    fn init(base: Base<Node3D>) -> Self {
        Self::create(base, Controls)
    }

    fn ready(&mut self) {
        let gimbal_h = self.gimbal_h.clone().upcast();
        self.base_mut().add_child(gimbal_h);

        let gimbal_v = self.gimbal_v.clone().upcast();
        self.gimbal_h.add_child(gimbal_v);

        let camera = self.camera.clone().upcast();
        self.gimbal_v.add_child(camera);
    }

    fn process(&mut self, delta: f64) {
        // get the camera's current horizontal and vertical rotation angles and assign them to local fields
        let cam_rot = self.controls.bind().get_camera_rotation();
        self.rot_h = cam_rot.x;
        self.rot_v = cam_rot.y;

        // lerp the the horizontal and vertical gimbals' rotations towards the corresponding rotation angles
        // note that we're using lerp instead of lerp_angle because the latter tries to determine the rotation
        // direction based checked the current and target values, which would cause the camera to twitch around
        // the current value and not reach the target value
        let y = lerpf(
            self.gimbal_h.get_rotation().y as f64,
            deg_to_rad(self.rot_h as f64),
            ROTATE_SPEED * delta,
        ) as f32;
        self.gimbal_h.rotate_y(y);

        let x = lerpf(
            self.gimbal_v.get_rotation().x as f64,
            deg_to_rad(self.rot_v as f64),
            ROTATE_SPEED * delta,
        ) as f32;
        self.gimbal_v.rotate_x(x);

        // lerp the camera's current local Z position towards the distance variable as determined by the
        // controls node's zoom scale value in the _process method
        self.camera.get_transform().origin.z = lerpf(
            self.camera.get_transform().origin.z as f64,
            DISTANCE,
            ZOOM_SPEED * delta,
        ) as f32;
    }
}
