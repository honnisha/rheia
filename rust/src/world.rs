use godot::prelude::*;

use crate::chunks::chunks_manager::ChunksManager;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct World {
    #[base]
    base: Base<Node>,
    camera: Option<Gd<Camera3D>>,
    chunks_manager: Option<ChunksManager>,
}

#[godot_api]
impl NodeVirtual for World {
    fn init(base: Base<Node>) -> Self {
        World {
            base,
            camera: None,
            chunks_manager: None,
        }
    }

    fn ready(&mut self) {
        self.camera = Some(self.base.get_parent().unwrap().get_node_as("Camera"));

        self.chunks_manager = Some(ChunksManager::new());

        godot_print!("World loaded;");
    }

    #[allow(unused_variables)]
    fn process(&mut self, delta: f64) {
        let camera = self.camera.as_deref_mut().unwrap();
        self.chunks_manager
            .as_mut()
            .unwrap()
            .update_camera_position(&mut self.base, camera.get_global_position());
    }
}
