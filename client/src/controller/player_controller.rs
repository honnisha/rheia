use common::network::messages::ClientMessages;
use godot::{
    engine::{Engine, InputEvent},
    prelude::*,
};
use std::fmt::{self, Display, Formatter};

use crate::{entities::position::GodotPositionConverter, main_scene::FloatType};

use super::handlers::freecam::FreeCameraHandler;

const CAMERA_PATH: &str = "Camera";
const DEBUG_INFO_PATH: &str = "DebugInfo";

#[derive(Clone, Copy, Debug, PartialEq, ToVariant, FromVariant)]
pub struct PlayerMovement {
    // Player object position
    position: Vector3,

    // vertical angle
    yaw: FloatType,

    // horizontal angle
    pitch: FloatType,
}

impl Display for PlayerMovement {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "(pos:{} yaw:{} pitch:{})", self.position, self.yaw, self.pitch)
    }
}

impl PlayerMovement {
    pub fn create(position: Vector3, yaw: FloatType, pitch: FloatType) -> Self {
        Self { position, yaw, pitch }
    }

    pub fn into_network(&self) -> ClientMessages {
        ClientMessages::PlayerMove {
            position: GodotPositionConverter::vec3_to_array(&self.position),
            yaw: self.yaw,
            pitch: self.pitch,
        }
    }
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct PlayerController {
    #[base]
    pub(crate) base: Base<Node>,
    camera: Gd<Camera3D>,
    handler: Option<FreeCameraHandler>,
}

impl PlayerController {
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
        Self {
            base,
            camera: load::<PackedScene>("res://scenes/camera_3d.tscn").instantiate_as::<Camera3D>(),
            handler: None,
        }
    }

    fn ready(&mut self) {}

    fn input(&mut self, event: Gd<InputEvent>) {
        if let Some(h) = self.handler.as_mut() {
            h.input(event, &mut self.camera);
        }
    }

    #[allow(unused_variables)]
    fn process(&mut self, delta: f64) {
        if Engine::singleton().is_editor_hint() {
            return;
        }
        if let Some(h) = self.handler.as_mut() {
            h.process(&mut self.base, delta, &mut self.camera);
        }
    }
}
