use common::{
    blocks::{
        block_info::{BlockIndexType, BlockInfo},
        block_type::{BlockContent, BlockType},
    },
    chunks::block_position::BlockPosition,
};
use godot::{
    classes::{Control, IControl, InputEvent, InputEventMouseButton, Material, MeshInstance3D},
    global::MouseButton,
    prelude::*,
};

use crate::{
    client_scripts::resource_manager::ResourceStorage,
    utils::textures::texture_mapper::TextureMapper,
    world::{
        block_storage::BlockStorage,
        chunks::{
            chunk_data_formatter::generate_single_block, mesh::mesh_generator::generate_chunk_geometry,
            objects_container::ObjectsContainer,
        },
    },
};

#[derive(Clone, Copy, Debug, PartialEq, GodotClass)]
#[class(init)]
pub struct BlockIconSelect {
    block_id: BlockIndexType,
}

impl BlockIconSelect {
    pub fn create(block_id: BlockIndexType) -> Self {
        Self { block_id }
    }

    pub fn get_block_id(&self) -> &BlockIndexType {
        &self.block_id
    }
}

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct BlockIcon {
    base: Base<Control>,

    block_id: Option<BlockIndexType>,

    #[export]
    pub block_anchor: Option<Gd<Node3D>>,

    #[export]
    camera: Option<Gd<Camera3D>>,
}

#[godot_api]
impl BlockIcon {
    #[func]
    fn on_gui_input(&mut self, event: Gd<InputEvent>) {
        let Some(block_id) = self.block_id.as_ref() else {
            return;
        };

        if let Ok(event) = event.clone().try_cast::<InputEventMouseButton>() {
            if event.get_button_index() == MouseButton::LEFT && event.is_pressed() {
                let icon = Gd::<BlockIconSelect>::from_init_fn(|_base| BlockIconSelect::create(block_id.clone()));
                self.base_mut().emit_signal("icon_clicked", &[icon.to_variant()]);
            }
        }
    }

    #[func]
    fn on_mouse_entered(&mut self) {
        let Some(block_id) = self.block_id.as_ref() else {
            return;
        };
        // log::info!("enter block_id: {}", block_id);
    }

    #[func]
    fn on_mouse_exited(&mut self) {
        let Some(block_id) = self.block_id.as_ref() else {
            return;
        };
        // log::info!("exit block_id: {}", block_id);
    }

    #[signal]
    fn icon_clicked();
}

impl BlockIcon {
    pub fn setup_icon(
        &mut self,
        block_id: BlockIndexType,
        block_type: &BlockType,
        material: &Gd<Material>,
        texture_mapper: &TextureMapper,
        block_storage: &BlockStorage,
        resource_storage: &ResourceStorage,
    ) {
        self.block_id = Some(block_id);
        let block_anchor = self.block_anchor.as_mut().unwrap();
        match block_type.get_block_content() {
            BlockContent::Texture {
                texture: _,
                side_texture: _,
                bottom_texture: _,
            } => {
                let block_info = BlockInfo::create(block_id, None);
                let bordered_chunk_data = generate_single_block(&block_type, &block_info);
                let geometry = generate_chunk_geometry(texture_mapper, &bordered_chunk_data, &block_storage);

                let mut mesh = MeshInstance3D::new_alloc();
                mesh.set_mesh(&geometry.mesh_ist);
                mesh.set_position(Vector3::new(-1.5, -1.5, -1.5));

                mesh.set_material_overlay(material);

                if let Some(camera) = self.camera.as_mut() {
                    camera.set_size(2.0);
                }
                block_anchor.add_child(&mesh);
            }
            BlockContent::ModelCube { model: _ } => {
                let mut objects_container = ObjectsContainer::new_alloc();
                let position = BlockPosition::new(0, 0, 0);
                objects_container
                    .bind_mut()
                    .create_block_model(&position, block_type, None, resource_storage)
                    .unwrap();
                objects_container.set_position(Vector3::new(-1.5, -1.5, -1.5));

                if let Some(camera) = self.camera.as_mut() {
                    camera.set_size(3.0);
                }
                block_anchor.add_child(&objects_container);
            }
        }
    }
}

#[godot_api]
impl IControl for BlockIcon {
    fn ready(&mut self) {
        let gd = self.base().to_godot();
        self.base_mut()
            .connect("gui_input", &Callable::from_object_method(&gd, "on_gui_input"));

        self.base_mut()
            .connect("mouse_entered", &Callable::from_object_method(&gd, "on_mouse_entered"));
        self.base_mut()
            .connect("mouse_exited", &Callable::from_object_method(&gd, "on_mouse_exited"));
    }
}
