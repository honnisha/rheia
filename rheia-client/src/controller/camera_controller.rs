use common::chunks::rotation::Rotation;
use godot::classes::{Camera3D, Sprite2D};
use godot::prelude::*;

use super::{
    controls::Controls,
    player_controller::{CAMERA_DISTANCE, CONTROLLER_CAMERA_OFFSET_RIGHT},
};

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
}

impl CameraController {
    pub fn create(base: Base<Node3D>, controls: Gd<Controls>) -> Self {
        let cross = load::<PackedScene>(CROSS_SCENE).instantiate_as::<Sprite2D>();

        Self {
            base,
            camera: Camera3D::new_alloc(),
            controls,
            cross,
        }
    }

    /// Horisontal degrees
    pub fn get_yaw(&self) -> f32 {
        self.base().get_rotation_degrees().y
    }

    /// Vertical degrees
    pub fn _get_pitch(&self) -> f32 {
        self.base().get_rotation_degrees().x
    }

    pub fn _get_camera(&self) -> &Gd<Camera3D> {
        &self.camera
    }

    pub fn rotate(&mut self, rotation: Rotation) {
        let mut r = self.base().get_rotation_degrees();
        r.x = rotation.yaw % 360.0;
        r.y = rotation.pitch % 360.0;
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
}

#[godot_api]
impl CameraController {
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

        let mut t = self.camera.get_transform();
        t.origin.z = CAMERA_DISTANCE;
        t.origin.x = CONTROLLER_CAMERA_OFFSET_RIGHT;
        self.camera.set_transform(t);
    }

    fn process(&mut self, _delta: f64) {
        let cam_rot = {
            let controls = self.controls.bind();
            controls.get_camera_rotation().clone()
        };

        self.rotate(Rotation::new(cam_rot.y, cam_rot.x));
    }
}
