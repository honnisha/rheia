use common::network::messages::ClientMessages;
use godot::{
    engine::{Engine, InputEvent},
    prelude::*,
};
use log::error;
use std::fmt::{self, Display, Formatter};

use crate::{entities::position::GodotPositionConverter, main_scene::FloatType, world::world_manager::WorldManager};

use super::{debug_info::DebugInfo, handlers::freecam::FreeCameraHandler};

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
    camera: Option<Gd<Camera3D>>,
    debug_info: Option<Gd<DebugInfo>>,
    handler: Option<FreeCameraHandler>,
}

impl PlayerController {
    pub fn update_debug(&mut self, world_manager: &WorldManager) {
        if let Some(d) = self.debug_info.as_mut() {
            let camera = self.camera.as_deref().unwrap();
            d.bind_mut().update_debug(world_manager, camera, &self.handler);
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
        handler.teleport(&mut self.camera.as_mut().unwrap(), position, yaw, pitch);
    }
}

#[godot_api]
impl PlayerController {
    #[signal]
    fn on_player_move();

    #[func]
    pub fn get_position(&self) -> Vector3 {
        let handler = self.handler.as_ref().unwrap();
        let camera = self.camera.as_deref().unwrap();
        handler.get_position(camera)
    }
}

#[godot_api]
impl NodeVirtual for PlayerController {
    fn init(base: Base<Node>) -> Self {
        PlayerController {
            base,
            camera: None,
            debug_info: None,
            handler: None,
        }
    }

    fn ready(&mut self) {
        if Engine::singleton().is_editor_hint() {
            return;
        }

        self.camera = Some(
            self.base
                .try_get_node_as::<Camera3D>(CAMERA_PATH)
                .expect("Camera not found"),
        );

        match self.base.try_get_node_as::<DebugInfo>(DEBUG_INFO_PATH) {
            Some(c) => {
                self.debug_info = Some(c);
            }
            None => {
                error!("DEBUG_INFO_PATH not found");
            }
        }
    }

    #[allow(unused_variables)]
    fn input(&mut self, event: Gd<InputEvent>) {
        if Engine::singleton().is_editor_hint() {
            return;
        }
        if let Some(h) = self.handler.as_mut() {
            h.input(event, &mut self.camera.as_mut().unwrap());
        }
    }

    #[allow(unused_variables)]
    fn process(&mut self, delta: f64) {
        if Engine::singleton().is_editor_hint() {
            return;
        }
        if let Some(h) = self.handler.as_mut() {
            h.process(&mut self.base, delta, &mut self.camera.as_mut().unwrap());
        }
    }
}
