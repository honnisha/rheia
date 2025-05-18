use super::look_at::LookAt;
use crate::{
    utils::{
        bridge::IntoGodotVector,
        primitives::{generate_lines, get_face_vector},
    },
    world::physics::{PhysicsType, get_degrees_from_normal},
};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(no_init, base=Node)]
pub struct BuildingVisualizer {
    base: Base<Node>,
    block_selection: Gd<Node3D>,
}

#[godot_api]
impl BuildingVisualizer {
    pub fn create(base: Base<Node>) -> Self {
        let mut selection = Node3D::new_alloc();

        let mesh = generate_lines(get_face_vector(), Color::from_rgb(0.0, 0.0, 0.0));
        selection.add_child(&mesh);
        selection.set_visible(false);

        Self {
            base,
            block_selection: selection,
        }
    }

    pub fn on_look_at_update(&mut self, new_look: Option<Gd<LookAt>>) {
        let Some(new_look) = new_look else {
            self.block_selection.set_visible(false);
            return;
        };
        let new_look = new_look.bind();
        match new_look.get_physics_type() {
            PhysicsType::ChunkMeshCollider(_chunk_position) => {
                let selected_block = new_look.get_cast_result().get_selected_block();
                self.block_selection.set_visible(true);
                self.block_selection
                    .set_global_position(selected_block.get_position().to_godot() + Vector3::new(0.5, 0.5, 0.5));
                self.block_selection
                    .set_rotation_degrees(get_degrees_from_normal(new_look.get_cast_result().normal.to_godot()));
            }
            PhysicsType::EntityCollider(_) => todo!(),
        }
    }
}

#[godot_api]
impl INode for BuildingVisualizer {
    fn ready(&mut self) {
        let block_selection = self.block_selection.clone();
        self.base_mut().add_child(&block_selection);
    }
}
