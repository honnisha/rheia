use godot::{engine::InputEvent, prelude::*};

use crate::main_scene::FloatType;

use super::handlers::freecam::FreeCameraHandler;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct PlayerController {
    #[base]
    pub(crate) base: Base<Node>,
    camera: Gd<Camera3D>,
    handler: Option<FreeCameraHandler>,
}

impl PlayerController {
    pub fn create(base: Base<Node>, camera: &Gd<Camera3D>) -> Self {
        Self {
            base,
            camera: camera.share(),
            handler: None,
        }
    }

    /// Handle network packet for changing position
    pub fn teleport(&mut self, position: Vector3, yaw: FloatType, pitch: FloatType) {
        let handler = match self.handler.as_mut() {
            Some(h) => h,
            None => {
                self.handler = Some(FreeCameraHandler::create());
                self.handler.as_mut().unwrap()
            }
        };
        handler.teleport(&mut self.camera, position, yaw, pitch);
    }

    pub fn get_handler(&self) -> Option<&FreeCameraHandler> {
        match self.handler.as_ref() {
            Some(h) => Some(h),
            None => None,
        }
    }
}

#[godot_api]
impl PlayerController {
    #[signal]
    fn on_player_move();

    #[func]
    pub fn get_position(&self) -> Vector3 {
        let handler = self.handler.as_ref().unwrap();
        handler.get_position(&self.camera)
    }
}

#[godot_api]
impl NodeVirtual for PlayerController {
    fn init(base: Base<Node>) -> Self {
        let camera = load::<PackedScene>("res://scenes/camera_3d.tscn").instantiate_as::<Camera3D>();
        Self::create(base, &camera)
    }

    fn ready(&mut self) {}

    fn input(&mut self, event: Gd<InputEvent>) {
        if let Some(h) = self.handler.as_mut() {
            h.input(event, &mut self.camera);
        }
    }

    #[allow(unused_variables)]
    fn process(&mut self, delta: f64) {
        if let Some(h) = self.handler.as_mut() {
            h.process(&mut self.base, delta, &mut self.camera);
        }
    }
}
