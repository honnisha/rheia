use super::{look_at::LookAt, selected_item::SelectedItem};
use crate::{
    scenes::components::block_mesh_storage::BlockMeshStorage,
    utils::{
        bridge::IntoGodotVector,
        primitives::{generate_lines, get_face_vector},
    },
    world::physics::{PhysicsType, get_degrees_from_normal},
};
use godot::{
    classes::{
        BaseMaterial3D, GeometryInstance3D, MeshInstance3D,
        base_material_3d::{BlendMode, Feature, Transparency},
    },
    prelude::*,
};

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
        selection.set_name("Selection");

        let mesh = generate_lines(get_face_vector(), Color::from_rgb(0.0, 0.0, 0.0));
        selection.add_child(&mesh);
        selection.set_visible(false);

        let mut block_preview_anchor = Node3D::new_alloc();
        block_preview_anchor.set_name("BlockPreviewAnchor");
        block_preview_anchor.set_visible(false);

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

                let selected_block = new_look.get_cast_result().get_selected_block();
                self.block_selection.set_visible(true);
                // +0.5 to place it on center of the block
                self.block_selection
                    .set_global_position(selected_block.get_position().to_godot() + Vector3::new(0.5, 0.5, 0.5));
                self.block_selection
                    .set_rotation_degrees(get_degrees_from_normal(new_look.get_cast_result().normal.to_godot()));

                // Block preview
                if self.selected_item.is_some() {
                    self.block_preview_anchor.set_visible(true);
                    let selected_block = new_look.get_cast_result().get_place_block();

                    // +0.5 to place it on center of the block
                    self.block_preview_anchor
                        .set_global_position(selected_block.get_position().to_godot() + Vector3::new(0.5, 0.5, 0.5));
                }
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
                                        let obj =
                                            self.block_preview_anchor.get_children().iter_shared().next().unwrap();
                                        let mut obj = obj.cast::<Node3D>();
                                        if let Some(face) = block_info.get_face() {
                                            let rotation = face.get_rotation();
                                            let mut r = obj.get_rotation_degrees();
                                            r.x = rotation.yaw % 360.0;
                                            r.y = rotation.pitch % 360.0;
                                            obj.set_rotation_degrees(r);
                                        }
                                    }
                                    return;
                                }
                            }
                        }
                    }

                    self.clear_block_preview_anchor();

                    let block_mesh_storage = self.block_mesh_storage.as_ref().unwrap();
                    let block_mesh_storage = block_mesh_storage.bind();
                    let mesh = block_mesh_storage.get_mesh(&block_info.get_id());
                    BuildingVisualizer::walk_change_color(mesh.clone(), Color::from_rgb(0.0, 1.0, 0.0), 0.5);
                    self.block_preview_anchor.add_child(&mesh);
                }
            },
            None => {
                self.clear_block_preview_anchor();
            }
        }
        self.selected_item = new_item;
    }

    fn change_color(obj: Gd<Node3D>, color: Color, alpha: f32) -> bool {
        if let Ok(mut obj) = obj.clone().try_cast::<GeometryInstance3D>() {
            obj.set_transparency(alpha);
            if let Some(material) = obj.get_material_overlay() {
                if let Ok(base_material) = material.try_cast::<BaseMaterial3D>() {
                    let mut new_material = base_material.duplicate().unwrap().cast::<BaseMaterial3D>();

                    new_material.set_feature(Feature::DETAIL, true);
                    new_material.set_detail_blend_mode(BlendMode::SUB);

                    let mut color = color.clone();
                    color.a = alpha;

                    new_material.set_albedo(color);
                    new_material.set_transparency(Transparency::DISABLED);
                    obj.set_material_overlay(&new_material);
                    return true;
                }
            }
        }

        if let Ok(mut obj) = obj.clone().try_cast::<MeshInstance3D>() {
            if let Some(mesh) = obj.get_mesh() {
                // mesh.set_local_to_scene(true);

                if let Some(material) = mesh.surface_get_material(0) {
                    if let Ok(base_material) = material.try_cast::<BaseMaterial3D>() {
                        // base_material.set_local_to_scene(true);

                        let mut new_material = base_material.duplicate().unwrap().cast::<BaseMaterial3D>();
                        let color = color.clone();
                        new_material.set_albedo(color);
                        new_material.set_transparency(Transparency::ALPHA);
                        // mesh.surface_set_material(0, &new_material);
                        obj.set_material_override(&new_material);
                        return true;
                    }
                }
            }
        }
        return false;
    }

    fn walk_change_color(obj: Gd<Node3D>, color: Color, alpha: f32) {
        if BuildingVisualizer::change_color(obj.clone(), color, alpha) {
            return;
        }
        for child in obj.clone().get_children().iter_shared() {
            if let Ok(child) = child.try_cast::<Node3D>() {
                BuildingVisualizer::walk_change_color(child.clone(), color, alpha);
            }
        }
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
