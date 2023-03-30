use godot::{engine::Engine, prelude::*};

use crate::{world::chunks::chunks_manager::ChunksManager, controller::player_controller::PlayerController};

pub mod blocks;
pub mod chunks;
pub mod world_generator;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct World {
    #[base]
    base: Base<Node>,
    chunks_manager: Option<ChunksManager>,
}

#[godot_api]
impl World {
    #[func]
    fn handle_camera_move(&mut self, global_position: Vector3) {
        self.chunks_manager
            .as_mut()
            .unwrap()
            .update_camera_position(&mut self.base, global_position);
    }
}

#[godot_api]
impl NodeVirtual for World {
    fn init(base: Base<Node>) -> Self {
        World {
            base,
            chunks_manager: None,
        }
    }

    fn ready(&mut self) {
        if Engine::singleton().is_editor_hint() {
            return;
        }

        let player_controller = self
            .base
            .get_parent()
            .unwrap()
            .try_get_node_as::<PlayerController>("PlayerController");
        if player_controller.is_some() {
            player_controller.unwrap().bind_mut().connect(
                "submit_camera_move".into(),
                Callable::from_object_method(self.base.share(), "handle_camera_move"),
                0,
            );
        } else {
            godot_error!("PlayerController element not found for World");
        }

        self.chunks_manager = Some(ChunksManager::new());
        godot_print!("World loaded;");
    }
}
