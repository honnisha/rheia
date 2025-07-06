use common::chunks::rotation::Rotation;
use godot::classes::{Camera3D, Sprite2D};
use godot::prelude::*;

use super::controls::Controls;

const CROSS_SCENE: &str = "res://scenes/cross.tscn";

pub struct RayDirection {
    pub dir: Vector3,
    pub from: Vector3,
    pub max_toi: f32,
}
#[derive(GodotClass)]
#[class(no_init, base=Node3D)]
pub struct CameraController {
    base: Base<Node3D>,
    camera: Gd<Camera3D>,
    controls: Gd<Controls>,
    cross: Gd<Sprite2D>,

    _stored_rotation: Option<Rotation>,
}

impl CameraController {
    pub fn create(base: Base<Node3D>, controls: Gd<Controls>) -> Self {
        let cross = load::<PackedScene>(CROSS_SCENE).instantiate_as::<Sprite2D>();

        Self {
            base,
            camera: Camera3D::new_alloc(),
            controls,
            cross,
            _stored_rotation: Default::default(),
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

    pub fn _get_camera(&self) -> &Gd<Camera3D> {
        &self.camera
    }

    pub fn rotate(&mut self, rotation: Rotation) {
        let mut r = self.base().get_rotation_degrees();
        r.y = rotation.yaw % 360.0;
        r.x = rotation.pitch % 360.0;
        self.base_mut().set_rotation_degrees(r);
    }

    pub fn get_ray_from_center(&self) -> RayDirection {
        let screen = self.camera.get_viewport().unwrap().get_visible_rect().size;

        let from = self.camera.project_ray_origin(screen / 2.0);
        let to = from + self.camera.project_ray_normal(screen / 2.0) * 10.0;

        let dir = to - from;
        let (dir, max_toi) = (dir.normalized(), dir.length());
        RayDirection { dir, from, max_toi }
    }

    pub fn set_camera_distance(&mut self, distance: f32, offset_right: f32) {
        let mut t = self.camera.get_transform();
        t.origin.z = distance;
        t.origin.x = offset_right;
        self.camera.set_transform(t);
    }
}

#[godot_api]
impl CameraController {
    #[signal]
    pub fn on_camera_rotation(yaw: f32, pitch: f32);

    #[func]
    fn on_viewport_size_changed(&mut self) {
        let screen = self.camera.get_viewport().unwrap().get_visible_rect().size;
        self.cross.set_position(screen * 0.5);
    }
}

#[godot_api]
impl INode3D for CameraController {
    fn ready(&mut self) {
        let camera = self.camera.clone();
        self.base_mut().add_child(&camera);

        let cross = self.cross.clone();
        self.base_mut().add_child(&cross);

        let screen = self.camera.get_viewport().unwrap().get_visible_rect().size;
        self.cross.set_position(screen * 0.5);

        self.base().get_tree().unwrap().get_root().unwrap().connect(
            "size_changed",
            &Callable::from_object_method(&self.base().to_godot(), "on_viewport_size_changed"),
        );
    }

    fn process(&mut self, _delta: f64) {
        let cam_rot = {
            let controls = self.controls.bind();
            controls.get_camera_rotation().clone()
        };

        let rotation = Rotation::new(cam_rot.x, cam_rot.y);
        let rotate_changed = match self._stored_rotation.as_ref() {
            Some(r) => *r != rotation,
            None => true,
        };
        if rotate_changed {
            self.rotate(rotation);
            self.signals().on_camera_rotation().emit(rotation.yaw, rotation.pitch);
            self._stored_rotation = Some(rotation);
        }
    }
}
