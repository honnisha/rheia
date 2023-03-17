use std::collections::HashMap;

use godot::prelude::*;

use crate::chunks::chunks_manager::ChunksManager;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct World {
    #[base]
    base: Base<Node>,
    camera: Option<Gd<Camera3D>>,
    chunks_manager: ChunksManager,
}

#[godot_api]
impl GodotExt for World {
    fn init(base: Base<Node>) -> Self {
        World {
            base,
            camera: None,
            chunks_manager: ChunksManager::new(),
        }
    }

    fn ready(&mut self) {
        //self.camera = Some(self.base.get_node_as("Camera"));
        godot_print!("World loaded;");
    }

    #[allow(unused_variables)]
    fn process(&mut self, delta: f64) {
        //let camera = self.camera.as_deref_mut().unwrap();
        self.chunks_manager.update_camera_position(&mut self.base, Vector3::default());
    }
}