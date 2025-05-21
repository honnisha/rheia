use super::{look_at::LookAt, selected_item::SelectedItem};
use crate::{
    scenes::components::block_mesh_storage::BlockMeshStorage,
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

    block_mesh_storage: Option<Gd<BlockMeshStorage>>,

    block_selection: Gd<Node3D>,

    selected_item: Option<SelectedItem>,
    block_preview_anchor: Gd<Node3D>,
}

#[godot_api]
impl BuildingVisualizer {
    pub fn create(base: Base<Node>) -> Self {
        let mut selection = Node3D::new_alloc();

        let mesh = generate_lines(get_face_vector(), Color::from_rgb(0.0, 0.0, 0.0));
        selection.add_child(&mesh);
        selection.set_visible(false);

        let block_preview_anchor = Node3D::new_alloc();

        Self {
            base,
            block_selection: selection,
            selected_item: None,
            block_mesh_storage: None,
            block_preview_anchor,
        }
    }

    pub fn set_block_mesh_storage(&mut self, block_mesh_storage: Gd<BlockMeshStorage>) {
        self.block_mesh_storage = Some(block_mesh_storage);
    }

    pub fn on_look_at_update(&mut self, new_look: Option<Gd<LookAt>>) {
        let Some(new_look) = new_look else {
            self.block_selection.set_visible(false);
            self.block_preview_anchor.set_visible(false);
            return;
        };
        let new_look = new_look.bind();
        match new_look.get_physics_type() {
            PhysicsType::ChunkMeshCollider(_chunk_position) => {
                // Activate block selection
                if self.selected_item.is_some() {
                    let selected_block = new_look.get_cast_result().get_selected_block();
                    self.block_selection.set_visible(true);
                    self.block_selection
                        .set_global_position(selected_block.get_position().to_godot() + Vector3::new(0.5, 0.5, 0.5));
                    self.block_selection
                        .set_rotation_degrees(get_degrees_from_normal(new_look.get_cast_result().normal.to_godot()));
                }
                self.block_preview_anchor.set_visible(true);
                let selected_block = new_look.get_cast_result().get_place_block();
                self.block_preview_anchor.set_visible(true);
                self.block_preview_anchor
                    .set_global_position(selected_block.get_position().to_godot() + Vector3::new(0.5, 0.5, 0.5));
            }
            PhysicsType::EntityCollider(_) => (),
        }
    }

    pub fn set_selected_item(&mut self, new_item: Option<SelectedItem>) {
        match new_item.as_ref() {
            Some(new_item) => match new_item {
                SelectedItem::BlockPlacing(block_info) => {
                    if let Some(selected_item) = self.selected_item.as_ref() {
                        match selected_item {
                            SelectedItem::BlockPlacing(old_block_info) => {
                                if old_block_info.get_id() == block_info.get_id() {
                                    if old_block_info.get_face() != block_info.get_face() {
                                        // Rotate existing object
                                        todo!();
                                    }
                                    return;
                                }
                            }
                            SelectedItem::BlockDestroy => (),
                        }
                    }

                    self.clear_block_preview_anchor();

                    let block_mesh_storage = self.block_mesh_storage.as_ref().unwrap();
                    let block_mesh_storage = block_mesh_storage.bind();
                    let mesh = block_mesh_storage.get_mesh(&block_info.get_id());
                    self.block_preview_anchor.add_child(&mesh);
                }
                SelectedItem::BlockDestroy => {
                    self.clear_block_preview_anchor();
                }
            },
            None => {
                self.clear_block_preview_anchor();
            }
        }
        self.selected_item = new_item;
    }

    fn clear_block_preview_anchor(&mut self) {
        if self.selected_item.is_some() {
            for mut child in self.block_preview_anchor.get_children().iter_shared() {
                child.queue_free();
            }
        }
    }
}

#[godot_api]
impl INode for BuildingVisualizer {
    fn ready(&mut self) {
        let block_selection = self.block_selection.clone();
        self.base_mut().add_child(&block_selection);

        let block_preview_anchor = self.block_preview_anchor.clone();
        self.base_mut().add_child(&block_preview_anchor);
    }
}
