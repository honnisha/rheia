use godot::{
    engine::{Engine, InputEvent},
    prelude::*,
};

use crate::world::world_manager::WorldManager;

use super::debug_info::DebugInfo;

const CAMERA_PATH: &str = "Camera";
const DEBUG_INFO_PATH: &str = "DebugInfo";

#[derive(GodotClass)]
#[class(base=Node)]
pub struct PlayerController {
    #[base]
    base: Base<Node>,
    camera: Option<Gd<Camera3D>>,
    debug_info: Option<Gd<DebugInfo>>,
}

impl PlayerController {
    pub fn teleport(&mut self, new_position: Vector3) {}

    pub fn update_debug(&mut self, world_manager: &WorldManager) {
        if self.camera.is_none() {
            return;
        }

        if let Some(d) = self.debug_info.as_mut() {
            let camera = self.camera.as_deref_mut().unwrap();
            d.bind_mut().update_debug(world_manager, camera);
        }
    }
}

#[godot_api]
impl PlayerController {
    // #[signal]
    // fn submit_camera_move();

        // if self.buffer_position.distance_to(camera_pos) > 0.1 {
        //     self.buffer_position = camera_pos;
        //     self.base
        //         .emit_signal("submit_camera_move".into(), &[camera_pos.to_variant()]);
        // }
}

#[godot_api]
impl NodeVirtual for PlayerController {
    fn init(base: Base<Node>) -> Self {
        PlayerController {
            base,
            camera: None,
            debug_info: None,
        }
    }

    fn ready(&mut self) {
        if Engine::singleton().is_editor_hint() {
            return;
        }

        match self.base.try_get_node_as::<Camera3D>(CAMERA_PATH) {
            Some(c) => {
                self.camera = Some(c);
            }
            None => {
                godot_error!("Camera element not found for PlayerController");
            }
        }

        match self.base.try_get_node_as::<DebugInfo>(DEBUG_INFO_PATH) {
            Some(c) => {
                self.debug_info = Some(c);
            }
            None => {
                godot_error!("Debug text element not found for PlayerController");
            }
        }
    }

    #[allow(unused_variables)]
    fn input(&mut self, event: Gd<InputEvent>) {}

    #[allow(unused_variables)]
    fn process(&mut self, delta: f64) {
        if Engine::singleton().is_editor_hint() {
            return;
        }
    }
}
